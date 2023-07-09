use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::{Arc, Mutex};

use flutter_windows_sys::{
    FlutterDesktopGpuSurfaceDescriptor, FlutterDesktopGpuSurfaceTextureConfig,
    FlutterDesktopGpuSurfaceType_kFlutterDesktopGpuSurfaceTypeD3d11Texture2D,
    FlutterDesktopGpuSurfaceType_kFlutterDesktopGpuSurfaceTypeDxgiSharedHandle,
    FlutterDesktopGpuSurfaceType_kFlutterDesktopGpuSurfaceTypeNone,
    FlutterDesktopTextureInfo__bindgen_ty_1,
    FlutterDesktopTextureRegistrarMarkExternalTextureFrameAvailable,
    FlutterDesktopTextureRegistrarRef, FlutterDesktopTextureRegistrarRegisterExternalTexture,
    FlutterDesktopTextureRegistrarUnregisterExternalTexture,
    FlutterDesktopTextureType_kFlutterDesktopGpuSurfaceTexture,
};

pub struct FlutterDesktopTextureRegistrar {
    ptr: FlutterDesktopTextureRegistrarRef,
    textures: Arc<Mutex<HashMap<i64, Box<FlutterDesktopTextureInfo>>>>,
}

unsafe impl Send for FlutterDesktopTextureRegistrar {}
unsafe impl Sync for FlutterDesktopTextureRegistrar {}

pub enum FlutterDesktopTextureInfo {
    // PixelBuffer,
    GpuSurface {
        surface_type: FlutterDesktopGpuSurfaceType,
        callback: Box<dyn Fn(usize, usize) -> &'static FlutterDesktopGpuSurfaceDescriptor>,
    },
}

pub enum FlutterDesktopGpuSurfaceType {
    None,
    DxgiSharedHandle,
    D3d11Texture2D,
}

impl FlutterDesktopTextureRegistrar {
    pub fn new(
        texture_registrar: FlutterDesktopTextureRegistrarRef,
    ) -> FlutterDesktopTextureRegistrar {
        FlutterDesktopTextureRegistrar {
            ptr: texture_registrar,
            textures: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register_external_texture(&self, info: Box<FlutterDesktopTextureInfo>) -> i64 {
        unsafe {
            let texture = FlutterDesktopTextureRegistrarRegisterExternalTexture(self.ptr, &match &*info {
                FlutterDesktopTextureInfo::GpuSurface {
                    surface_type,
                    ..
                } => flutter_windows_sys::FlutterDesktopTextureInfo {
                    type_: FlutterDesktopTextureType_kFlutterDesktopGpuSurfaceTexture,
                    __bindgen_anon_1: FlutterDesktopTextureInfo__bindgen_ty_1 {
                        gpu_surface_config: FlutterDesktopGpuSurfaceTextureConfig {
                            struct_size: std::mem::size_of::<FlutterDesktopGpuSurfaceTextureConfig>(),
                            type_: match surface_type {
                                FlutterDesktopGpuSurfaceType::None => {
                                    FlutterDesktopGpuSurfaceType_kFlutterDesktopGpuSurfaceTypeNone
                                }
                                FlutterDesktopGpuSurfaceType::DxgiSharedHandle => FlutterDesktopGpuSurfaceType_kFlutterDesktopGpuSurfaceTypeDxgiSharedHandle,
                                FlutterDesktopGpuSurfaceType::D3d11Texture2D => FlutterDesktopGpuSurfaceType_kFlutterDesktopGpuSurfaceTypeD3d11Texture2D,
                            },
                            callback: Some(gpu_surface_config_callback),
                            user_data: &*info as *const FlutterDesktopTextureInfo as *mut c_void,
                        },
                    },
                },
            });

            self.textures.lock().unwrap().insert(texture, info);

            texture
        }
    }

    pub fn unregister_external_texture(&self, texture_id: i64) {
        unsafe extern "C" fn callback_<F: Fn()>(user_data: *mut c_void) {
            Box::from_raw(user_data.cast::<F>())();
        }

        fn get_callback<F: Fn()>(_: &F) -> unsafe extern "C" fn(*mut c_void) {
            callback_::<F>
        }

        let textures = self.textures.clone();
        let callback = Box::leak(Box::new(move || {
            textures.lock().unwrap().remove(&texture_id);
        }));

        unsafe {
            FlutterDesktopTextureRegistrarUnregisterExternalTexture(
                self.ptr,
                texture_id,
                Some(get_callback(callback)),
                callback as *mut _ as _,
            );
        }
    }

    pub fn mark_external_texture_frame_available(&self, texture_id: i64) {
        unsafe {
            FlutterDesktopTextureRegistrarMarkExternalTextureFrameAvailable(self.ptr, texture_id)
        };
    }
}

unsafe extern "C" fn gpu_surface_config_callback(
    width: usize,
    height: usize,
    user_data: *mut c_void,
) -> *const flutter_windows_sys::FlutterDesktopGpuSurfaceDescriptor {
    let info = (user_data as *const FlutterDesktopTextureInfo)
        .as_ref()
        .unwrap();

    match info {
        FlutterDesktopTextureInfo::GpuSurface { callback, .. } => callback(width, height),
    }
}
