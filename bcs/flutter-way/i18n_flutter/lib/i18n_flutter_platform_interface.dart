import 'package:plugin_platform_interface/plugin_platform_interface.dart';

import 'i18n_flutter_method_channel.dart';

abstract class I18nFlutterPlatform extends PlatformInterface {
  /// Constructs a I18nFlutterPlatform.
  I18nFlutterPlatform() : super(token: _token);

  static final Object _token = Object();

  static I18nFlutterPlatform _instance = MethodChannelI18nFlutter();

  /// The default instance of [I18nFlutterPlatform] to use.
  ///
  /// Defaults to [MethodChannelI18nFlutter].
  static I18nFlutterPlatform get instance => _instance;

  /// Platform-specific implementations should set this with their own
  /// platform-specific class that extends [I18nFlutterPlatform] when
  /// they register themselves.
  static set instance(I18nFlutterPlatform instance) {
    PlatformInterface.verifyToken(instance, _token);
    _instance = instance;
  }

  Future<String?> getPlatformVersion() {
    throw UnimplementedError('platformVersion() has not been implemented.');
  }
}
