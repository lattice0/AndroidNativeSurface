use glium::{implement_vertex, index::PrimitiveType, program, uniform, Surface, Texture2d};
use log::info;
use android_native_surface::pure_gl::PureGlRenderer;

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
    let context = glium::glutin::ContextBuilder::new().with_vsync(true).build_windowed(wb, (1280,720)).unwrap();
    let context = unsafe { context.make_current() }.unwrap();
    let mut gl_context = PureGlRenderer::new_from_context(
        &context,
    );
    gl_context.draw();
    context.swap_buffers().unwrap();
    std::thread::sleep(std::time::Duration::from_secs(50));
}
fn main() {
    println!("hello");
    render_into();
}
