use std::collections::BTreeMap;
use std::ffi::{c_void, CStr, CString};
use std::io::{Cursor, Write};
use std::ptr;
use std::sync::Mutex;

use flutter_codec::EncodableValue;
use flutter_windows_sys::{
    FlutterDesktopMessage, FlutterDesktopMessageResponseHandle, FlutterDesktopMessengerRef,
    FlutterDesktopMessengerSend, FlutterDesktopMessengerSendResponse,
    FlutterDesktopMessengerSetCallback,
};

pub use flutter_codec as codec;

pub type FlutterDesktopMessengerCallback =
    Box<dyn Fn(&str, EncodableValue, FlutterDesktopMessengerReply)>;

pub struct FlutterDesktopMessenger {
    ptr: FlutterDesktopMessengerRef,
    callbacks: Mutex<BTreeMap<CString, FlutterDesktopMessengerCallback>>,
}

unsafe impl Send for FlutterDesktopMessenger {}
unsafe impl Sync for FlutterDesktopMessenger {}

impl FlutterDesktopMessenger {
    pub fn new(messenger: FlutterDesktopMessengerRef) -> FlutterDesktopMessenger {
        FlutterDesktopMessenger {
            ptr: messenger,
            callbacks: Mutex::new(BTreeMap::new()),
        }
    }

    pub fn call(&self, channel: &str, method_name: &str, args: &EncodableValue) {
        let channel = CString::new(channel).unwrap();
        let mut message_bytes = vec![];
        let mut cursor = Cursor::new(&mut message_bytes);
        flutter_codec::write_value(&mut cursor, &EncodableValue::Str(method_name)).unwrap();
        flutter_codec::write_value(&mut cursor, args).unwrap();
        unsafe {
            FlutterDesktopMessengerSend(
                self.ptr,
                channel.as_ptr(),
                message_bytes.as_ptr(),
                message_bytes.len(),
            );
        }
    }

    pub fn send(&self, channel: &str, message: &EncodableValue) {
        let channel = CString::new(channel).unwrap();
        let mut message_bytes = vec![];
        let mut cursor = Cursor::new(&mut message_bytes);
        flutter_codec::write_value(&mut cursor, message).unwrap();
        unsafe {
            FlutterDesktopMessengerSend(
                self.ptr,
                channel.as_ptr(),
                message_bytes.as_ptr(),
                message_bytes.len(),
            );
        }
    }

    pub fn set_callback(
        &self,
        channel: &str,
        callback: impl Fn(&str, EncodableValue, FlutterDesktopMessengerReply) + 'static,
    ) {
        self.set_callback_impl(channel, Box::new(callback))
    }

    fn set_callback_impl(&self, channel: &str, callback: FlutterDesktopMessengerCallback) {
        let channel = CString::new(channel).unwrap();

        let mut callbacks = self.callbacks.lock().unwrap();
        callbacks.insert(channel.clone(), Box::new(callback));

        let callback = callbacks.get(&channel).unwrap();

        unsafe {
            FlutterDesktopMessengerSetCallback(
                self.ptr,
                channel.as_ptr(),
                Some(messenger_callback),
                callback as *const _ as _,
            )
        };
    }
}

impl Drop for FlutterDesktopMessenger {
    fn drop(&mut self) {
        unsafe {
            for channel in self.callbacks.lock().unwrap().keys() {
                FlutterDesktopMessengerSetCallback(
                    self.ptr,
                    channel.as_ptr(),
                    None,
                    ptr::null_mut(),
                );
            }
        }
    }
}

unsafe extern "C" fn messenger_callback(
    messenger: FlutterDesktopMessengerRef,
    message: *const FlutterDesktopMessage,
    user_data: *mut c_void,
) {
    let callback =
        user_data as *const Box<dyn Fn(&str, EncodableValue, FlutterDesktopMessengerReply)>;
    let callback = callback.as_ref().unwrap();

    let buf = std::slice::from_raw_parts((*message).message, (*message).message_size);
    let mut cursor = Cursor::new(buf);

    let method_name = flutter_codec::read_value(&mut cursor).unwrap();
    let method_args = flutter_codec::read_value(&mut cursor).unwrap();

    let EncodableValue::Str(method_name) = method_name else {
        let channel = CStr::from_ptr((*message).channel);
        let channel = channel.to_str().unwrap_or("invalid_channel");
        eprintln!("[method_call({channel})] invalid method name: {method_name:?}");
        return;
    };

    let response_handle = (*message).response_handle;
    let reply = FlutterDesktopMessengerReply {
        messenger,
        response_handle,
    };

    callback(method_name, method_args, reply);
}

pub struct FlutterDesktopMessengerReply {
    messenger: FlutterDesktopMessengerRef,
    response_handle: *const FlutterDesktopMessageResponseHandle,
}

impl FlutterDesktopMessengerReply {
    pub fn success(&self, value: &EncodableValue) {
        let mut bytes = vec![];
        let mut cursor = Cursor::new(&mut bytes);
        cursor.write_all(&[0]).unwrap();
        flutter_codec::write_value(&mut cursor, value).unwrap();
        unsafe {
            FlutterDesktopMessengerSendResponse(
                self.messenger,
                self.response_handle,
                bytes.as_ptr(),
                bytes.len(),
            );
        }
    }

    pub fn not_implemented(&self) {
        unsafe {
            FlutterDesktopMessengerSendResponse(
                self.messenger,
                self.response_handle,
                std::ptr::null(),
                0,
            );
        }
    }
}
