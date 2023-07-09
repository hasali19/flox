pub mod window_binding;

use std::env;
use std::ffi::CString;
use std::mem::transmute;
use std::ptr::null;

use flutter_desktop_messenger::FlutterDesktopMessenger;
use flutter_windows_sys::{
    FlutterDesktopEngineCreate, FlutterDesktopEngineGetMessenger, FlutterDesktopEngineProperties,
    FlutterDesktopEngineRef, FlutterDesktopEngineReloadSystemFonts,
    FlutterDesktopViewControllerCreate, FlutterDesktopViewControllerDestroy,
    FlutterDesktopViewControllerGetEngine, FlutterDesktopViewControllerGetView,
    FlutterDesktopViewControllerHandleTopLevelWindowProc, FlutterDesktopViewControllerRef,
    FlutterDesktopViewGetHWND, FlutterDesktopViewRef,
};
use windows::w;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};

pub struct FlutterDesktopViewController {
    controller: FlutterDesktopViewControllerRef,
}

extern "C" {
    fn register_plugins(engine: FlutterDesktopEngineRef);
}

impl FlutterDesktopViewController {
    pub fn new(width: i32, height: i32) -> FlutterDesktopViewController {
        let args = env::args().skip(1).collect::<Vec<_>>();

        let args = args
            .into_iter()
            .map(|arg| CString::new(arg).unwrap())
            .collect::<Vec<_>>();

        let mut argv = args.iter().map(|arg| arg.as_ptr()).collect::<Vec<_>>();

        let engine_properties = FlutterDesktopEngineProperties {
            assets_path: w!("data\\flutter_assets").as_ptr(),
            icu_data_path: w!("data\\icudtl.dat").as_ptr(),
            aot_library_path: w!("data\\app.so").as_ptr(),
            dart_entrypoint: null(),
            dart_entrypoint_argc: args.len() as i32,
            dart_entrypoint_argv: argv.as_mut_ptr(),
        };

        let engine = unsafe { FlutterDesktopEngineCreate(&engine_properties) };
        let view_controller = unsafe { FlutterDesktopViewControllerCreate(width, height, engine) };

        unsafe { register_plugins(engine) };

        FlutterDesktopViewController {
            controller: view_controller,
        }
    }

    pub fn engine(&self) -> FlutterDesktopEngine {
        FlutterDesktopEngine {
            engine: unsafe { FlutterDesktopViewControllerGetEngine(self.controller) },
        }
    }

    pub fn view(&self) -> FlutterDesktopView {
        FlutterDesktopView {
            view: unsafe { FlutterDesktopViewControllerGetView(self.controller) },
        }
    }

    pub fn handle_top_level_window_proc(
        &self,
        hwnd: HWND,
        umsg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> Option<LRESULT> {
        let mut result = LRESULT::default();

        let handled = unsafe {
            FlutterDesktopViewControllerHandleTopLevelWindowProc(
                self.controller,
                transmute(hwnd),
                umsg,
                transmute(wparam),
                transmute(lparam),
                &mut result as *mut LRESULT as *mut flutter_windows_sys::LRESULT,
            )
        };

        handled.then_some(result)
    }
}

impl Drop for FlutterDesktopViewController {
    fn drop(&mut self) {
        unsafe { FlutterDesktopViewControllerDestroy(self.controller) };
    }
}

pub struct FlutterDesktopEngine {
    engine: FlutterDesktopEngineRef,
}

impl FlutterDesktopEngine {
    pub fn create_messenger(&self) -> FlutterDesktopMessenger {
        FlutterDesktopMessenger::new(unsafe { FlutterDesktopEngineGetMessenger(self.engine) })
    }

    pub fn reload_system_fonts(&self) {
        unsafe { FlutterDesktopEngineReloadSystemFonts(self.engine) };
    }
}

pub struct FlutterDesktopView {
    view: FlutterDesktopViewRef,
}

impl FlutterDesktopView {
    pub fn hwnd(&self) -> HWND {
        HWND(unsafe { FlutterDesktopViewGetHWND(self.view) } as isize)
    }
}
