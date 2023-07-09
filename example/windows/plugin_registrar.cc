#include <flutter/plugin_registry.h>
#include <flutter_windows.h>

#include "flutter/generated_plugin_registrant.h"

class FlutterPluginRegistry : public flutter::PluginRegistry {
 public:
  FlutterPluginRegistry(FlutterDesktopEngineRef engine) : engine_(engine) {}

  FlutterDesktopPluginRegistrarRef GetRegistrarForPlugin(
      const std::string& plugin_name) {
    return FlutterDesktopEngineGetPluginRegistrar(engine_, plugin_name.c_str());
  }

 private:
  FlutterDesktopEngineRef engine_ = nullptr;
};

extern "C" __declspec(dllexport) void register_plugins(
    FlutterDesktopEngineRef engine) {
  FlutterPluginRegistry registry(engine);
  RegisterPlugins(&registry);
}
