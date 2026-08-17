import 'package:flutter_test/flutter_test.dart';
import 'package:i18n_flutter/i18n_flutter.dart';
import 'package:i18n_flutter/i18n_flutter_platform_interface.dart';
import 'package:i18n_flutter/i18n_flutter_method_channel.dart';
import 'package:plugin_platform_interface/plugin_platform_interface.dart';

class MockI18nFlutterPlatform
    with MockPlatformInterfaceMixin
    implements I18nFlutterPlatform {
  @override
  Future<String?> getPlatformVersion() => Future.value('42');
}

void main() {
  final I18nFlutterPlatform initialPlatform = I18nFlutterPlatform.instance;

  test('$MethodChannelI18nFlutter is the default instance', () {
    expect(initialPlatform, isInstanceOf<MethodChannelI18nFlutter>());
  });

  test('getPlatformVersion', () async {
    I18nFlutter i18nFlutterPlugin = I18nFlutter();
    MockI18nFlutterPlatform fakePlatform = MockI18nFlutterPlatform();
    I18nFlutterPlatform.instance = fakePlatform;

    expect(await i18nFlutterPlugin.getPlatformVersion(), '42');
  });
}
