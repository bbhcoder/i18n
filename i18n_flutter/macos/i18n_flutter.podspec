#
# To learn more about a Podspec see http://guides.cocoapods.org/syntax/podspec.html.
# Run `pod lib lint i18n_flutter.podspec` to validate before publishing.
#
Pod::Spec.new do |s|
  s.name             = 'i18n_flutter'
  s.version          = '0.2.1'
  s.summary          = 'A blazing fast, cross-platform i18n engine using Rust under the hood.'
  s.description      = <<-DESC
A new Flutter plugin project that provides blazing fast internationalization capabilities natively on Android, iOS, Windows, macOS, Linux, and Web using WebAssembly.
                       DESC
  s.homepage         = 'https://github.com/bbhcoder/i18n'
  s.license          = { :file => '../LICENSE' }
  s.author           = { 'Arsalan Shoaei' => 'arsalanshoaei@gmail.com' }

  s.source           = { :path => '.' }
  s.source_files = 'i18n_flutter/Sources/i18n_flutter/**/*'

  # 👇 این خط اضافه شد تا فلاتر بتونه فایل dylib دانلود شده در ریشه پروژه رو برای مک لینک کنه
  s.vendored_libraries = 'libencheco_i18n.dylib'

  # If your plugin requires a privacy manifest, for example if it collects user
  # data, update the PrivacyInfo.xcprivacy file to describe your plugin's
  # privacy impact, and then uncomment this line. For more information,
  # see https://developer.apple.com/documentation/bundleresources/privacy_manifest_files
  # s.resource_bundles = {'i18n_flutter_privacy' => ['i18n_flutter/Sources/i18n_flutter/PrivacyInfo.xcprivacy']}

  s.dependency 'FlutterMacOS'

  s.platform = :osx, '10.11'
  s.pod_target_xcconfig = { 'DEFINES_MODULE' => 'YES' }
  s.swift_version = '5.0'
end