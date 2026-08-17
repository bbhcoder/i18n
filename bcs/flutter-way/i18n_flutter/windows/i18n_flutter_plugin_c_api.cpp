#include "include/i18n_flutter/i18n_flutter_plugin_c_api.h"

#include <flutter/plugin_registrar_windows.h>

#include "i18n_flutter_plugin.h"

void I18nFlutterPluginCApiRegisterWithRegistrar(
    FlutterDesktopPluginRegistrarRef registrar) {
  i18n_flutter::I18nFlutterPlugin::RegisterWithRegistrar(
      flutter::PluginRegistrarManager::GetInstance()
          ->GetRegistrar<flutter::PluginRegistrarWindows>(registrar));
}
