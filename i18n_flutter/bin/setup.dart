// ignore_for_file: avoid_print
import 'dart:io';

const String version = 'v0.2.1'; // دقت کن که به ورژن تگ گیت‌هابت بخوره
const String githubUrl = 'https://github.com/bbhcoder/i18n/releases/download/$version';

const String coiServiceWorkerContent = '''
/*! coi-serviceworker v0.1.6 - Guido Zuidhof, licensed under MIT */
if(typeof window==="undefined"){self.addEventListener("install",()=>self.skipWaiting());self.addEventListener("activate",e=>e.waitUntil(self.clients.claim()));self.addEventListener("message",e=>{if(e.data==="deregister"){self.registration.unregister().then(()=>self.clients.matchAll()).then(clients=>{clients.forEach(client=>client.navigate(client.url))})}});self.addEventListener("fetch",function(e){if(e.request.cache==="only-if-cached"&&e.request.mode!=="same-origin"){return}e.respondWith(fetch(e.request).then(r=>{if(r.status===0){return r}const headers=new Headers(r.headers);headers.set("Cross-Origin-Embedder-Policy","require-corp");headers.set("Cross-Origin-Opener-Policy","same-origin");return new Response(r.body,{status:r.status,statusText:r.statusText,headers})}).catch(e=>console.error(e)))});}else{(()=>{const e=window.document.currentScript!=undefined?window.document.currentScript.src:"";if(window.sessionStorage&&window.sessionStorage.getItem("coiReloadedBySelf")){window.sessionStorage.removeItem("coiReloadedBySelf")}else if(e){if(window.crossOriginIsolated!==false)return;window.sessionStorage.setItem("coiReloadedBySelf","true");if(window.isSecureContext){window.navigator.serviceWorker.register(e).then(e=>{e.addEventListener("updatefound",()=>{console.log("Reloading page to make use of updated COOP/COEP Service Worker.");window.sessionStorage.removeItem("coiReloadedBySelf");window.location.reload()});if(e.active&&!window.crossOriginIsolated){console.log("Reloading page to make use of COOP/COEP Service Worker.");window.location.reload()}},e=>{console.error("COOP/COEP Service Worker failed to register:",e)})}else if(!window.isSecureContext){console.log("COOP/COEP Service Worker not registered, a secure context is required.")}}})()}
''';

Future<void> main() async {
  print('🚀 Starting i18n_flutter automated setup...\n');

  await setupDesktopBinaries();
  await setupAndroidBinaries();
  await setupIosBinaries();
  await setupWebEnvironment();

  print('\n✅ Setup completed successfully!');
  print('🎉 You can now run your app on ANY platform without extra configuration.');
}

/// ---------------------------------------------------------
/// بخش اول: دسکتاپ (ویندوز، لینوکس، مک)
/// ---------------------------------------------------------
Future<void> setupDesktopBinaries() async {
  print('💻 Checking Desktop environment...');
  String downloadName = '';
  String saveName = '';
  
  if (Platform.isWindows) {
    downloadName = 'windows-x64-encheco_i18n.dll';
    saveName = 'encheco_i18n.dll';
  } else if (Platform.isLinux) {
    downloadName = 'linux-x64-libencheco_i18n.so';
    saveName = 'libencheco_i18n.so';
  } else if (Platform.isMacOS) {
    if (Platform.version.contains('arm64')) {
      downloadName = 'macos-arm64-libencheco_i18n.dylib';
    } else {
      downloadName = 'macos-x64-libencheco_i18n.dylib';
    }
    saveName = 'libencheco_i18n.dylib';
  } else {
    print('   - Not a desktop platform. Skipping desktop binary download.');
    return;
  }

  final savePath = '${Directory.current.path}/$saveName';
  print('   - Downloading binary for ${Platform.operatingSystem}...');
  await downloadFile('$githubUrl/$downloadName', savePath);
}

/// ---------------------------------------------------------
/// بخش دوم: اندروید
/// ---------------------------------------------------------
Future<void> setupAndroidBinaries() async {
  final androidDir = Directory('${Directory.current.path}/android/app/src/main/jniLibs');
  
  if (!Directory('${Directory.current.path}/android').existsSync()) {
    print('📱 Android folder not found. Skipping Android setup.');
    return;
  }

  print('\n📱 Configuring Android environment...');
  
  final targets = {
    'arm64-v8a': 'android-arm64-libencheco_i18n.so',
    'armeabi-v7a': 'android-arm-libencheco_i18n.so',
    'x86_64': 'android-x64-libencheco_i18n.so',
  };

  for (var entry in targets.entries) {
    final arch = entry.key;
    final fileName = entry.value;
    final targetDir = Directory('${androidDir.path}/$arch');
    
    if (!targetDir.existsSync()) {
      targetDir.createSync(recursive: true);
    }

    print('   - Downloading Android binary for $arch...');
    await downloadFile('$githubUrl/$fileName', '${targetDir.path}/libencheco_i18n.so');
  }
}

/// ---------------------------------------------------------
/// بخش سوم: iOS
/// ---------------------------------------------------------
Future<void> setupIosBinaries() async {
  final iosDir = Directory('${Directory.current.path}/ios');
  if (!iosDir.existsSync()) {
    print('🍏 iOS folder not found. Skipping iOS setup.');
    return;
  }

  print('\n🍏 Configuring iOS environment...');
  final zipName = 'ios-encheco_i18n.xcframework.zip';
  final zipPath = '${iosDir.path}/$zipName';

  print('   - Downloading XCFramework for iOS...');
  await downloadFile('$githubUrl/$zipName', zipPath);

  if (Platform.isMacOS) {
    print('   - Extracting XCFramework...');
    final result = Process.runSync('unzip', ['-o', zipPath, '-d', iosDir.path]);
    if (result.exitCode != 0) {
      print('     ❌ Failed to extract zip: ${result.stderr}');
    } else {
      print('     ✔️ XCFramework extracted successfully.');
      File(zipPath).deleteSync(); // حذف فایل زیپ پس از استخراج موفق
    }
  } else {
    print('   - ⚠️ Note: Extraction skipped because you are not on macOS.');
  }
}

/// ---------------------------------------------------------
/// بخش چهارم: وب (WASM)
/// ---------------------------------------------------------
Future<void> setupWebEnvironment() async {
  final webDir = Directory('${Directory.current.path}/web');
  
  if (!webDir.existsSync()) {
    print('\n🌐 Web folder not found. Skipping web setup.');
    return;
  }

  print('\n🌐 Configuring Web environment...');

  final pkgDir = Directory('${webDir.path}/pkg');
  if (!pkgDir.existsSync()) {
    pkgDir.createSync(recursive: true);
  }

  print('   - Downloading WebAssembly files...');
  await downloadFile('$githubUrl/encheco_i18n.js', '${pkgDir.path}/encheco_i18n.js');
  await downloadFile('$githubUrl/encheco_i18n_bg.wasm', '${pkgDir.path}/encheco_i18n_bg.wasm');

  final swFile = File('${webDir.path}/coi-serviceworker.js');
  if (!swFile.existsSync()) {
    swFile.writeAsStringSync(coiServiceWorkerContent);
    print('   - Created coi-serviceworker.js for Cross-Origin headers.');
  } else {
    print('   - coi-serviceworker.js already exists.');
  }

  final indexHtml = File('${webDir.path}/index.html');
  if (indexHtml.existsSync()) {
    String htmlContent = indexHtml.readAsStringSync();
    if (!htmlContent.contains('coi-serviceworker.js')) {
      htmlContent = htmlContent.replaceFirst(
        '<head>', 
        '<head>\n  <script src="coi-serviceworker.js"></script>'
      );
      indexHtml.writeAsStringSync(htmlContent);
      print('   - Injected Service Worker into web/index.html');
    }
  }
}

/// ---------------------------------------------------------
/// متد کمکی: دانلود فایل
/// ---------------------------------------------------------
Future<void> downloadFile(String url, String savePath) async {
  try {
    final request = await HttpClient().getUrl(Uri.parse(url));
    final response = await request.close();
    if (response.statusCode == 200) {
      final file = File(savePath);
      await response.pipe(file.openWrite());
      print('     ✔️ Downloaded to: $savePath');
    } else {
      print('     ❌ Failed to download. HTTP Status: ${response.statusCode}');
    }
  } catch (e) {
    print('     ❌ Error downloading file: $e');
  }
}