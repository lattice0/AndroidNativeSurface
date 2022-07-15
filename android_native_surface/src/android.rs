use android_logger::{Config, FilterBuilder};
use glium::{implement_vertex, index::PrimitiveType, program, uniform, Surface, Texture2d};
use jni::{
    objects::JClass,
    sys::{jint, jlong, jobject, JNI_VERSION_1_6},
    JNIEnv, JavaVM,
};
use log::{debug, info, Level};
use ndk::{native_window::NativeWindow, surface_texture::SurfaceTexture};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use jni::objects::JObject;
use crate::pure_gl::PureGlRenderer;

#[no_mangle]
pub extern "system" fn JNI_OnLoad(_java_vm: JavaVM, _reserved: *const libc::c_void) -> jint {
    android_logger::init_once(
        Config::default()
            .with_min_level(Level::Trace) // limit log level
            .with_tag("glium@android") // logs will show under mytag tag
            .with_filter(
                FilterBuilder::new()
                    .parse("debug,hello::crate=error")
                    .build(),
            ),
    );
    info!("started android logging");
    JNI_VERSION_1_6
}

#[no_mangle]
pub extern "system" fn Java_rust_androidnativesurface_MainActivity_00024Companion_renderToSurfaceTexture(
    env: JNIEnv,
    _class: JClass,
    surface_texture: JObject,
) {
    debug!("Java SurfaceTexture: {:?}", surface_texture);
    let surface_texture = unsafe {
        SurfaceTexture::from_surface_texture(env.get_native_interface(), surface_texture.into_inner()).unwrap()
    };
    let native_window = surface_texture.acquire_native_window().unwrap();
    let context = glium::glutin::ContextBuilder::new().build_windowed(
        native_window,
        /* TODO: Size currently not needed */ (0, 0),
    ).unwrap();
    //let native_window = surface_texture.acquire_native_window().unwrap();
    let context = unsafe { context.make_current() }.unwrap();
    let mut gl_context = PureGlRenderer::new_from_context(
        &context,
    );
    gl_context.draw();
    //render_to_native_window(native_window)
}
