// i18n_flutter/bin/setup.dart
// ignore_for_file: avoid_print
import 'dart:io';

const String version = 'v0.1.9';
const String githubUrl = 'https://github.com/bbhcoder/i18n/releases/download/$version';

// کد جاوااسکریپت Service Worker برای تزریق هدرهای Cross-Origin
const String coiServiceWorkerContent = '''
/*! coi-serviceworker v0.1.6 - Guido Zuidhof, licensed under MIT */
if(typeof window==="undefined"){self.addEventListener("install",()=>self.skipWaiting());self.addEventListener("activate",e=>e.waitUntil(self.clients.claim()));self.addEventListener("message",e=>{if(e.data==="deregister"){self.registration.unregister().then(()=>self.clients.matchAll()).then(clients=>{clients.forEach(client=>client.navigate(client.url))})}});self.addEventListener("fetch",function(e){if(e.request.cache==="only-if-cached"&&e.request.mode!=="same-origin"){return}e.respondWith(fetch(e.request).then(r=>{if(r.status===0){return r}const headers=new Headers(r.headers);headers.set("Cross-Origin-Embedder-Policy","require-corp");headers.set("Cross-Origin-Opener-Policy","same-origin");return new Response(r.body,{status:r.status,statusText:r.statusText,headers})}).catch(e=>console.error(e)))});}else{(()=>{const e=window.document.currentScript!=undefined?window.document.currentScript.src:"";if(window.sessionStorage&&window.sessionStorage.getItem("coiReloadedBySelf")){window.sessionStorage.removeItem("coiReloadedBySelf")}else if(e){if(window.crossOriginIsolated!==false)return;window.sessionStorage.setItem("coiReloadedBySelf","true");if(window.isSecureContext){window.navigator.serviceWorker.register(e).then(e=>{e.addEventListener("updatefound",()=>{console.log("Reloading page to make use of updated COOP/COEP Service Worker.");window.sessionStorage.removeItem("coiReloadedBySelf");window.location.reload()});if(e.active&&!window.crossOriginIsolated){console.log("Reloading page to make use of COOP/COEP Service Worker.");window.location.reload()}},e=>{console.error("COOP/COEP Service Worker failed to register:",e)})}else if(!window.isSecureContext){console.log("COOP/COEP Service Worker not registered, a secure context is required.")}}})()}
''';

Future<void> main() async {
  print('🚀 Starting i18n_flutter automated setup...\n');

  await setupDesktopBinaries();
  await setupWebEnvironment();

  print('\n✅ Setup completed successfully!');
  print('🎉 You can now run your app on Desktop and Web without any extra configuration.');
}

/// ---------------------------------------------------------
/// بخش اول: دانلود باینری‌های دسکتاپ (از کد قبلی شما)
/// ---------------------------------------------------------
Future<void> setupDesktopBinaries() async {
  print('💻 Checking Desktop environment...');
  String fileName = '';
  
  if (Platform.isWindows) {
    fileName = 'windows-x64-encheco_i18n.dll';
  } else if (Platform.isLinux) {
    fileName = 'linux-x64-libencheco_i18n.so';
  } else if (Platform.isMacOS) {
    fileName = 'macos-x64-libencheco_i18n.dylib';
  } else {
    print('   - Mobile platform detected (Android/iOS). Native builds are handled via Gradle/CocoaPods. Skipping binary download.');
    return;
  }

  final url = '$githubUrl/$fileName';
  final savePath = '${Directory.current.path}/$fileName';

  print('   - Downloading binary for ${Platform.operatingSystem}...');
  await downloadFile(url, savePath);
}

/// ---------------------------------------------------------
/// بخش دوم: پیکربندی اتوماتیک محیط وب (WASM و هدرها)
/// ---------------------------------------------------------
Future<void> setupWebEnvironment() async {
  final webDir = Directory('${Directory.current.path}/web');
  
  if (!webDir.existsSync()) {
    print('🌐 Web folder not found. Skipping web setup.');
    return;
  }

  print('\n🌐 Configuring Web environment...');

  // ۱. ساخت پوشه pkg و دانلود فایل‌های WASM
  final pkgDir = Directory('${webDir.path}/pkg');
  if (!pkgDir.existsSync()) {
    pkgDir.createSync(recursive: true);
  }

  print('   - Downloading WebAssembly files...');
  await downloadFile('$githubUrl/encheco_i18n.js', '${pkgDir.path}/encheco_i18n.js');
  await downloadFile('$githubUrl/encheco_i18n_bg.wasm', '${pkgDir.path}/encheco_i18n_bg.wasm');

  // ۲. ایجاد فایل Service Worker
  final swFile = File('${webDir.path}/coi-serviceworker.js');
  if (!swFile.existsSync()) {
    swFile.writeAsStringSync(coiServiceWorkerContent);
    print('   - Created coi-serviceworker.js for Cross-Origin headers.');
  } else {
    print('   - coi-serviceworker.js already exists.');
  }

  // ۳. تزریق Service Worker به index.html کاربر
  final indexHtml = File('${webDir.path}/index.html');
  if (indexHtml.existsSync()) {
    String htmlContent = indexHtml.readAsStringSync();
    
    // بررسی اینکه آیا قبلاً اسکریپت اضافه شده یا نه
    if (!htmlContent.contains('coi-serviceworker.js')) {
      // تزریق اسکریپت درست بعد از تگ <head>
      htmlContent = htmlContent.replaceFirst(
        '<head>', 
        '<head>\n  <script src="coi-serviceworker.js"></script>'
      );
      indexHtml.writeAsStringSync(htmlContent);
      print('   - Injected Service Worker into web/index.html');
    } else {
      print('   - Service Worker is already injected in web/index.html');
    }
  } else {
    print('   - ⚠️ web/index.html not found! Cannot inject Service Worker.');
  }
}

/// ---------------------------------------------------------
/// متد کمکی برای دانلود فایل‌ها
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
      print('     ❌ Failed to download. HTTP Status: ${response.statusCode} - URL: $url');
    }
  } catch (e) {
    print('     ❌ Error downloading file: $e');
  }
}