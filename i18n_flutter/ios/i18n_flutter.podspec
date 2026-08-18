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
  s.dependency 'Flutter'
  s.platform = :ios, '13.0'

  # 👇 این خط اضافه شد تا فلاتر بتونه فایل XCFramework دانلود شده رو پیدا و لینک کنه
  s.vendored_frameworks = 'encheco_i18n.xcframework'

  # Flutter.framework does not contain a i386 slice.
  s.pod_target_xcconfig = { 'DEFINES_MODULE' => 'YES', 'EXCLUDED_ARCHS[sdk=iphonesimulator*]' => 'i386' }
  s.swift_version = '5.0'

  # If your plugin requires a privacy manifest, for example if it uses any
  # required reason APIs, update the PrivacyInfo.xcprivacy file to describe your
  # plugin's privacy impact, and then uncomment this line. For more information,
  # see https://developer.apple.com/documentation/bundleresources/privacy_manifest_files
  # s.resource_bundles = {'i18n_flutter_privacy' => ['i18n_flutter/Sources/i18n_flutter/PrivacyInfo.xcprivacy']}
end