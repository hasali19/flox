#![windows_subsystem = "windows"]

use flutter_windows::window_binding::FlutterWindowBinding;
use flutter_windows::FlutterDesktopViewController;
use raw_window_handle::HasRawWindowHandle;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::platform::windows::IconExtWindows;
use winit::window::{Icon, WindowBuilder};

pub fn main() {
    let mut event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Flox Example")
        .with_inner_size(LogicalSize::new(1280, 720))
        .with_min_inner_size(LogicalSize::new(300, 300))
        .with_visible(false)
        .with_window_icon(Some(
            Icon::from_resource(101 /* IDI_APP_ICON (resource.h) */, None).unwrap(),
        ))
        .build(&event_loop)
        .unwrap();

    window.set_visible(true);

    let window_size = window.inner_size();

    let view_controller = Box::new(FlutterDesktopViewController::new(
        window_size.width as i32,
        window_size.height as i32,
    ));

    let _binding = FlutterWindowBinding::new(&view_controller, window.raw_window_handle());

    event_loop.run_return(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Event::WindowEvent {
            window_id: _,
            event: WindowEvent::CloseRequested,
        } = event
        {
            *control_flow = ControlFlow::Exit;
        }
    });
}
