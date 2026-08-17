import 'i18n_flutter_platform_interface.dart';

class I18nFlutter {
  Future<String?> getPlatformVersion() {
    return I18nFlutterPlatform.instance.getPlatformVersion();
  }
}
