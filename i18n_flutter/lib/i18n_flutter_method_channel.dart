import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';

import 'i18n_flutter_platform_interface.dart';

/// An implementation of [I18nFlutterPlatform] that uses method channels.
class MethodChannelI18nFlutter extends I18nFlutterPlatform {
  /// The method channel used to interact with the native platform.
  @visibleForTesting
  final methodChannel = const MethodChannel('i18n_flutter');

  @override
  Future<String?> getPlatformVersion() async {
    final version = await methodChannel.invokeMethod<String>(
      'getPlatformVersion',
    );
    return version;
  }
}
