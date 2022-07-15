//use super::pure_gl_utils::gl;
use glium::glutin::{ContextError, PossiblyCurrent};
use log::info;
use std::{ffi::CStr, string::FromUtf8Error};

pub mod gl {
    pub use self::Gles2 as Gl;
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

pub struct PureGlRenderer {
    pub(crate) gl: gl::Gl,
}

impl PureGlRenderer {
    pub fn new_from_context(
        gl_context: &glium::glutin::Context<PossiblyCurrent>,
    ) -> PureGlRenderer {
        let gl = gl::Gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);
        let version = unsafe {
            let data = CStr::from_ptr(gl.GetString(gl::VERSION) as *const _)
                .to_bytes()
                .to_vec();
            String::from_utf8(data).unwrap()
        };

        info!("OpenGL version {}", version);

        unsafe {
            let vs = gl.CreateShader(gl::VERTEX_SHADER);
            gl.ShaderSource(
                vs,
                1,
                [VIDEO_VERTEX_SHADER_300_ES.as_ptr() as *const _].as_ptr(),
                std::ptr::null(),
            );
            gl.CompileShader(vs);
            //shader_info(&gl, vs, gl::COMPILE_STATUS, "vertex shader compile")?;

            let fs = gl.CreateShader(gl::FRAGMENT_SHADER);
            gl.ShaderSource(
                fs,
                1,
                [PLANAR_FRAGMENT_SHADER_300_ES.as_ptr() as *const _].as_ptr(),
                std::ptr::null(),
            );
            gl.CompileShader(fs);
            //shader_info(&gl, fs, gl::COMPILE_STATUS, "fragment shader compile")?;

            let program = gl.CreateProgram();
            gl.AttachShader(program, vs);
            gl.AttachShader(program, fs);
            gl.LinkProgram(program);
            //program_info(&gl, program, gl::LINK_STATUS, "program link")?;

            gl.UseProgram(program);

            let position_attribute =
                gl.GetAttribLocation(program, b"position\0".as_ptr() as *const _);
            //get_attribute_info(position_attribute, "position")?;

            let color_attribute = gl.GetAttribLocation(program, b"color\0".as_ptr() as *const _);
            //get_attribute_info(color_attribute, "color")?;

            if gl.BindVertexArray.is_loaded() {
                let mut vao = std::mem::zeroed();
                gl.GenVertexArrays(1, &mut vao);
                gl.BindVertexArray(vao);
            } else {
                panic!("vertex bind error");
            }

            let mut vb = std::mem::zeroed();
            gl.GenBuffers(1, &mut vb);
            gl.BindBuffer(gl::ARRAY_BUFFER, vb);
            gl.BufferData(
                gl::ARRAY_BUFFER,
                (VERTEX_DATA.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                VERTEX_DATA.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl.VertexAttribPointer(
                position_attribute as gl::types::GLuint,
                3,
                gl::FLOAT,
                0,
                5 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                std::ptr::null(),
            );

            /*
            gl.VertexAttribPointer(
                color_attribute as gl::types::GLuint,
                2,
                gl::FLOAT,
                0,
                5 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                (3 * std::mem::size_of::<f32>()) as *const () as *const _,
            );
            */
            gl.EnableVertexAttribArray(position_attribute as gl::types::GLuint);
            //gl.EnableVertexAttribArray(color_attribute as gl::types::GLuint);
        }
        info!("end creation PureGLRenderer");
        PureGlRenderer { gl }
    }

    pub fn unload(&self) {
        todo!("todo unload gl");
    }

    pub fn parse_frame<'a>(
        &mut self,
    ) -> Result<(), ()> {
        Ok(())
    }

    pub fn draw<'a>(
        &mut self,
    ) -> std::result::Result<(), ()> {
        unsafe {
            self.gl.ClearColor(0.0, 0.0, 1.0, 1.0);
            self.gl.Clear(gl::COLOR_BUFFER_BIT);
            self.gl.DrawArrays(gl::TRIANGLE_STRIP, 0, 3);
        }
        info!("did draw!");
        Ok(())
    }
}


#[rustfmt::skip]
static VERTEX_DATA: [f32; 20] = [
    -1.0, -1.0,  0.0,  0.0,  1.0,
    1.0, -1.0,  0.0,  1.0,  1.0,
    -1.0,  1.0,  0.0,  0.0,  0.0,
    1.0,  1.0,  0.0,  1.0,  0.0,
];

pub const VIDEO_VERTEX_SHADER_300_ES: &[u8] = b"#version 300 es
layout (location = 0) in vec3 position;
//layout (location = 1) in vec2 color;

//out vec2 TexCoord;

void main()
{
    gl_Position = vec4(position, 1.0);
    //TexCoord = vec2(color.x, color.y);
}
\0";

pub const PLANAR_FRAGMENT_SHADER_300_ES: &[u8] = b"#version 300 es

#ifdef GL_ES
// Set default precision to medium, this is needed for GL_ES (android)
precision mediump int;
precision mediump float;
#endif

//uniform sampler2D tex_y;
//uniform sampler2D tex_u;
//uniform sampler2D tex_v;

//in vec2 TexCoord;
out vec4 FragColor;

void main()
{
    //vec3 yuv;
    //vec4 rgba;

    //yuv.r = texture(tex_y, TexCoord).r - 0.0625;
    //yuv.g = texture(tex_u, TexCoord).r - 0.5;
    //yuv.b = texture(tex_v, TexCoord).r - 0.5;

    //rgba.r = yuv.r + 1.596 * yuv.b;
    //rgba.g = yuv.r - 0.813 * yuv.b - 0.391 * yuv.g;
    //rgba.b = yuv.r + 2.018 * yuv.g;
    //rgba.a = 1.0;

    //FragColor = rgba;
    FragColor = vec4(1.0,0.0,0.0,1.0);
}
\0";
