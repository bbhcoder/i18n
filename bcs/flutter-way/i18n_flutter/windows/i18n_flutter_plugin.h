#ifndef FLUTTER_PLUGIN_I18N_FLUTTER_PLUGIN_H_
#define FLUTTER_PLUGIN_I18N_FLUTTER_PLUGIN_H_

#include <flutter/method_channel.h>
#include <flutter/plugin_registrar_windows.h>

#include <memory>

namespace i18n_flutter {

class I18nFlutterPlugin : public flutter::Plugin {
 public:
  static void RegisterWithRegistrar(flutter::PluginRegistrarWindows *registrar);

  I18nFlutterPlugin();

  virtual ~I18nFlutterPlugin();

  // Disallow copy and assign.
  I18nFlutterPlugin(const I18nFlutterPlugin&) = delete;
  I18nFlutterPlugin& operator=(const I18nFlutterPlugin&) = delete;

  // Called when a method is called on this plugin's channel from Dart.
  void HandleMethodCall(
      const flutter::MethodCall<flutter::EncodableValue> &method_call,
      std::unique_ptr<flutter::MethodResult<flutter::EncodableValue>> result);
};

}  // namespace i18n_flutter

#endif  // FLUTTER_PLUGIN_I18N_FLUTTER_PLUGIN_H_
