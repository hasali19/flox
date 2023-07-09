use std::cell::RefCell;
use std::collections::BTreeMap;

use flutter_windows_sys::{
    FlutterDesktopPluginRegistrarRef, FlutterDesktopPluginRegistrarSetDestructionHandler,
};

thread_local! {
    static DESTRUCTION_HANDLERS: RefCell<BTreeMap<FlutterDesktopPluginRegistrarRef, Box<dyn FnOnce()>>> = RefCell::new(BTreeMap::new());
}

pub fn add_destruction_callback(
    registrar: FlutterDesktopPluginRegistrarRef,
    handler: Box<dyn FnOnce()>,
) {
    DESTRUCTION_HANDLERS.with(|handlers| {
        handlers.borrow_mut().insert(registrar, handler);
    })
}

extern "C" fn on_destroy(registrar: FlutterDesktopPluginRegistrarRef) {
    DESTRUCTION_HANDLERS.with(|handlers| {
        if let Some(handler) = handlers.borrow_mut().remove(&registrar) {
            handler();
        }
    });
}

pub unsafe fn set_registrar_destruction_handler(registrar: FlutterDesktopPluginRegistrarRef) {
    FlutterDesktopPluginRegistrarSetDestructionHandler(registrar, Some(on_destroy));
}
