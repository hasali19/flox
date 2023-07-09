use flutter_windows_sys::{FlutterDesktopViewGetHWND, FlutterDesktopViewRef};
use windows::Win32::Foundation::HWND;

pub struct FlutterDesktopView {
    ptr: FlutterDesktopViewRef,
}

impl FlutterDesktopView {
    pub(crate) fn new(view: FlutterDesktopViewRef) -> FlutterDesktopView {
        FlutterDesktopView { ptr: view }
    }

    pub fn hwnd(&self) -> HWND {
        HWND(unsafe { FlutterDesktopViewGetHWND(self.ptr) } as isize)
    }
}
