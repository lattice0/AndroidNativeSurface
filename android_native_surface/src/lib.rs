use glium::backend::glutin::android_surface_texture::SurfaceBacked;
use glium::{implement_vertex, IndexBuffer, program, Surface, uniform, VertexBuffer};
use glium::index::PrimitiveType;
use glutin::ContextBuilder;
use glutin::dpi::PhysicalSize;
use glutin::event_loop::EventLoop;
use jni::{
    objects::{JClass, JObject},
    JNIEnv,
};
use log::{debug, Level};
use ndk::{surface_texture::SurfaceTexture};

#[no_mangle]
pub extern "system" fn Java_rust_androidnativesurface_MainActivity_00024Companion_init(
    _env: JNIEnv,
    _class: JClass,
) {
    android_logger::init_once(android_logger::Config::default().with_min_level(Level::Trace));
}

#[no_mangle]
pub extern "system" fn Java_rust_androidnativesurface_MainActivity_00024Companion_renderToSurfaceTexture(
    env: JNIEnv,
    _class: JClass,
    surface_texture: JObject,
) {
    debug!("Java SurfaceTexture: {:?}", surface_texture);

    let surface_texture = unsafe {
        SurfaceTexture::from_surface_texture(
            env.get_native_interface(),
            surface_texture.into_inner(),
        )
        .unwrap()
    };

    render_to_native_window(surface_texture)
}

fn render_to_native_window(surface_texture: SurfaceTexture) {
    debug!("{:?}", surface_texture);
    let size = PhysicalSize::new(1280,720);
    debug!("44");
    let el = EventLoop::<()>::with_user_event();
    debug!("46");
    let context = ContextBuilder::new().build_headless(&el,size).unwrap();
    let texture_id = 0;
    debug!("49");
    let display = SurfaceBacked::new(context, surface_texture, texture_id).unwrap();
    debug!("51");
    // building the vertex buffer, which contains all the vertices that we will draw
    let vertex_buffer = {
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 2],
            color: [f32; 3],
        }

        implement_vertex!(Vertex, position, color);

        VertexBuffer::new(
            &display,
            &[
                Vertex {
                    position: [-0.5, -0.5],
                    color: [0.0, 1.0, 0.0],
                },
                Vertex {
                    position: [0.0, 0.5],
                    color: [0.0, 0.0, 1.0],
                },
                Vertex {
                    position: [0.5, -0.5],
                    color: [1.0, 0.0, 0.0],
                },
            ],
        )
        .unwrap()
    };
    debug!("81");

    // building the index buffer
    let index_buffer =
        IndexBuffer::new(&display, PrimitiveType::TrianglesList, &[0u16, 1, 2]).unwrap();
    debug!("86");

    // compiling shaders and linking them together
    let program = program!(&display,
        140 => {
            vertex: "
                #version 140
                uniform mat4 matrix;
                in vec2 position;
                in vec3 color;
                out vec3 vColor;
                void main() {
                    gl_Position = vec4(position, 0.0, 1.0) * matrix;
                    vColor = color;
                }
            ",

            fragment: "
                #version 140
                in vec3 vColor;
                out vec4 f_color;
                void main() {
                    f_color = vec4(vColor, 1.0);
                }
            "
        },

        110 => {
            vertex: "
                #version 110
                uniform mat4 matrix;
                attribute vec2 position;
                attribute vec3 color;
                varying vec3 vColor;
                void main() {
                    gl_Position = vec4(position, 0.0, 1.0) * matrix;
                    vColor = color;
                }
            ",

            fragment: "
                #version 110
                varying vec3 vColor;
                void main() {
                    gl_FragColor = vec4(vColor, 1.0);
                }
            ",
        },

        100 => {
            vertex: "
                #version 100
                uniform lowp mat4 matrix;
                attribute lowp vec2 position;
                attribute lowp vec3 color;
                varying lowp vec3 vColor;
                void main() {
                    gl_Position = vec4(position, 0.0, 1.0) * matrix;
                    vColor = color;
                }
            ",

            fragment: "
                #version 100
                varying lowp vec3 vColor;
                void main() {
                    gl_FragColor = vec4(vColor, 1.0);
                }
            ",
        },
    )
    .unwrap();
    debug!("158");

    // Here we draw the black background and triangle to the screen using the previously
    // initialized resources.
    //
    // In this case we use a closure for simplicity, however keep in mind that most serious
    // applications should probably use a function that takes the resources as an argument.
    let draw = move || {
        // building the uniforms
        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ]
        };

        // drawing a frame
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        debug!("179");
        let e = target
            .draw(
                &vertex_buffer,
                &index_buffer,
                &program,
                &uniforms,
                &Default::default(),
            );
        if let Err(e) = e {
            debug!("draw error: {:?}", e);
        }
        debug!("189");
        target.finish().unwrap();
    };
    debug!("192");

    // Draw the triangle to the screen.
    draw();
    debug!("196");
}

