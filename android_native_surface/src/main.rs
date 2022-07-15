use glium::{implement_vertex, index::PrimitiveType, program, uniform, Surface, Texture2d};
use log::info;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

pub fn render_into() {
    let event_loop = glium::glutin::event_loop::EventLoop::new();
    let wb = glium::glutin::window::WindowBuilder::new()
        .build(&event_loop)
        .unwrap();
    let cb = glium::glutin::ContextBuilder::new().with_vsync(true);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let width = 1000;
    let height = 1000;

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

    info!("conna compile program");
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
}
fn main() {
    println!("hello");
    render_into();
}
