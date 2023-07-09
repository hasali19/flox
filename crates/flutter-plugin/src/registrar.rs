use std::sync::Arc;

use flutter_windows_sys::{
    FlutterDesktopPluginRegistrarGetMessenger, FlutterDesktopPluginRegistrarGetView,
    FlutterDesktopPluginRegistrarRef, FlutterDesktopRegistrarGetTextureRegistrar,
};

use crate::messenger::FlutterDesktopMessenger;
use crate::texture_registrar::FlutterDesktopTextureRegistrar;
use crate::view::FlutterDesktopView;

pub struct FlutterDesktopPluginRegistrar {
    _ptr: FlutterDesktopPluginRegistrarRef,
    messenger: Arc<FlutterDesktopMessenger>,
    view: FlutterDesktopView,
    texture_registrar: Arc<FlutterDesktopTextureRegistrar>,
}

impl FlutterDesktopPluginRegistrar {
    #[doc(hidden)]
    pub unsafe fn new(
        registrar: FlutterDesktopPluginRegistrarRef,
    ) -> FlutterDesktopPluginRegistrar {
        let messenger = FlutterDesktopPluginRegistrarGetMessenger(registrar);
        let view = FlutterDesktopPluginRegistrarGetView(registrar);
        let texture_registrar = FlutterDesktopRegistrarGetTextureRegistrar(registrar);
        FlutterDesktopPluginRegistrar {
            _ptr: registrar,
            messenger: Arc::new(FlutterDesktopMessenger::new(messenger)),
            view: FlutterDesktopView::new(view),
            texture_registrar: Arc::new(FlutterDesktopTextureRegistrar::new(texture_registrar)),
        }
    }

    pub fn messenger(&self) -> &Arc<FlutterDesktopMessenger> {
        &self.messenger
    }

    pub fn view(&self) -> &FlutterDesktopView {
        &self.view
    }

    pub fn texture_registrar(&self) -> &Arc<FlutterDesktopTextureRegistrar> {
        &self.texture_registrar
    }
}
