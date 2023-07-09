#[doc(hidden)]
pub mod destruction_handler;
pub mod messenger;
pub mod registrar;
pub mod texture_registrar;
pub mod view;

pub use flutter_codec as codec;
pub use flutter_windows_sys as sys;
#[doc(hidden)]
pub use flutter_windows_sys::FlutterDesktopPluginRegistrarRef;
#[doc(hidden)]
pub use paste::paste;

pub trait FlutterDesktopPlugin {
    fn register_with_registrar(registrar: &registrar::FlutterDesktopPluginRegistrar);
}

#[macro_export]
macro_rules! flutter_plugin {
    ($name:ident) => {
        $crate::paste! {
            #[no_mangle]
            pub unsafe extern "C" fn [<$name RegisterWithRegistrar>](registrar_ref: $crate::FlutterDesktopPluginRegistrarRef) {
                $crate::destruction_handler::set_registrar_destruction_handler(registrar_ref);

                let registrar = $crate::registrar::FlutterDesktopPluginRegistrar::new(registrar_ref);
                <$name as $crate::FlutterDesktopPlugin>::register_with_registrar(&registrar);

                $crate::destruction_handler::add_destruction_callback(
                    registrar_ref,
                    Box::new(|| drop(registrar)),
                );
            }
        }
    };
}
