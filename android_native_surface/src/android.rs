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
    render_to_native_window(native_window)
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

fn render_to_native_window(native_window: NativeWindow) {
    std::thread::spawn(move || {
        let width = 1000;
        let height = 1000;
        let context = glium::glutin::ContextBuilder::new()
            .build_windowed(native_window, (width, height))
            .unwrap();
        let display = glium::Display::from_gl_window(context).unwrap();

        let vertex_buffer_1 = {
            glium::VertexBuffer::new(
                &display,
                &[
                    Vertex {
                        position: [-1.0, -1.0],
                        tex_coords: [0.0, 0.0],
                    },
                    Vertex {
                        position: [-1.0, 1.0],
                        tex_coords: [0.0, 1.0],
                    },
                    Vertex {
                        position: [1.0, 1.0],
                        tex_coords: [1.0, 1.0],
                    },
                    Vertex {
                        position: [1.0, -1.0],
                        tex_coords: [1.0, 0.0],
                    },
                ],
            )
                .unwrap()
        };

        let index_buffer =
            glium::IndexBuffer::new(&display, PrimitiveType::TriangleStrip, &[1 as u16, 2, 0, 3])
                .unwrap();

        info!("compiling program");
        let program = program!(&display,

            300 es => {
                vertex: "#version 300 es

                in vec2 position;
                in vec2 tex_coords;

                out vec2 v_tex_coords;

                void main() {
                    v_tex_coords = tex_coords;
                    gl_Position = vec4(position, 0.0, 1.0);
                }",

                fragment: "#version 300 es
                #ifdef GL_ES
                // Set default precision to medium
                precision mediump int;
                precision mediump float;
                #endif
                uniform sampler2D tex;

                in vec2 v_tex_coords;
                out vec4 FragColor;

                void main() {
                    vec4 a = texture(tex, v_tex_coords);
                    FragColor = vec4(a.r,0.0,0.0,1.0);
                    //Uncomment to see that at least rendering the red square works
                    //FragColor = vec4(1.0,0.0,0.0,1.0);
                }",
            },
        );
        let r = program;
        info!("program result: {:?}", r);
        let program = r.unwrap();

        let mipmap = glium::texture::MipmapsOption::NoMipmap;
        let format = glium::texture::UncompressedFloatFormat::U8;
        let width = 1000;
        let height = 1000;

        let draw = || {
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 0.0);
            let texture =
                Texture2d::empty_with_format(&display, format, mipmap, width, height).unwrap();
            let mut data: Vec<Vec<u8>> = Vec::new();
            for i in 0..width {
                let mut v: Vec<u8> = Vec::new();
                for j in 0..height {
                    v.push((i * j % 255) as u8);
                }
                data.push(v);
            }

            texture.write(
                glium::Rect {
                    left: 0,
                    bottom: 0,
                    width,
                    height,
                },
                data,
            );
            let uniforms = uniform! {
                tex: texture
            };
            let r = target.draw(
                &vertex_buffer_1,
                &index_buffer,
                &program,
                &uniforms,
                &Default::default(),
            );
            info!("draw error? {:?}", r);
            let r = target.finish();
            info!("target finish error? {:?}", r);
        };

        loop {
            draw();
            std::thread::sleep(std::time::Duration::from_secs(2));
        }
    });
}

