// i18n_flutter/bin/setup.dart
import 'dart:io';

const String version = 'v0.1.4';
const String githubUrl = 'https://github.com/bbhcoder/i18n/releases/download/$version';

Future main() async {
  print('🚀 Starting i18n_engine binary setup...');

  String fileName = '';
  if (Platform.isWindows) {
    fileName = 'windows-x64-encheco_i18n.dll';
  } else if (Platform.isLinux) {
    fileName = 'linux-x64-libencheco_i18n.so';
  } else if (Platform.isMacOS) {
    // برای سادگی در این مرحله فقط x64 مک رو در نظر می‌گیریم
    fileName = 'macos-x64-libencheco_i18n.dylib';
  } else {
    print('⚠️ Please build natively for mobile (Android/iOS) or add their specific fetch logic.');
    return;
  }

  final url = '$githubUrl/$fileName';
  final savePath = '${Directory.current.path}/$fileName';

  print('⬇️ Downloading binary for ${Platform.operatingSystem}...');
  print('🔗 URL: $url');

  try {
    final request = await HttpClient().getUrl(Uri.parse(url));
    final response = await request.close();

    if (response.statusCode == 200) {
      final file = File(savePath);
      await response.pipe(file.openWrite());
      print('✅ Successfully downloaded binary to: $savePath');
    } else {
      print('❌ Failed to download binary. Status Code: ${response.statusCode}');
    }
  } catch (e) {
    print('❌ Error downloading binary: $e');
  }
}