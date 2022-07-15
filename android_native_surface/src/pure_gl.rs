use glium::glutin::{ContextError, PossiblyCurrent};
use log::info;
use std::{ffi::CStr, string::FromUtf8Error};

pub mod gl {
    pub use self::Gles2 as Gl;
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

pub struct PureGlRenderer {
    pub(crate) gl: gl::Gl,
    pub(crate) tex_id: Option<gl::types::GLuint>,
    pub(crate) tex_location: Option<gl::types::GLint>,
    pub(crate) width: u32,
    pub(crate) height: u32
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

        let width = 300;
        let height = 300;
        println!("OpenGL version {}", version);
        info!("OpenGL version {}", version);

        let mut tex_id: gl::types::GLuint = 0;
        let mut tex_location: gl::types::GLint = 0;
        unsafe {
            let vs = gl.CreateShader(gl::VERTEX_SHADER);
            gl.ShaderSource(
                vs,
                1,
                [VS_SRC.as_ptr() as *const _].as_ptr(),
                std::ptr::null(),
            );
            gl.CompileShader(vs);
            shader_info(&gl, vs, gl::COMPILE_STATUS, "vertex shader compile");

            let fs = gl.CreateShader(gl::FRAGMENT_SHADER);
            gl.ShaderSource(
                fs,
                1,
                [FS_SRC.as_ptr() as *const _].as_ptr(),
                std::ptr::null(),
            );
            gl.CompileShader(fs);
            shader_info(&gl, fs, gl::COMPILE_STATUS, "fragment shader compile");

            let program = gl.CreateProgram();
            gl.AttachShader(program, vs);
            gl.AttachShader(program, fs);
            gl.LinkProgram(program);
            gl.UseProgram(program);

            let mut vb = std::mem::zeroed();
            gl.GenBuffers(1, &mut vb);
            gl.BindBuffer(gl::ARRAY_BUFFER, vb);
            gl.BufferData(
                gl::ARRAY_BUFFER,
                (VERTEX_DATA.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                VERTEX_DATA.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            if gl.BindVertexArray.is_loaded() {
                let mut vao = std::mem::zeroed();
                gl.GenVertexArrays(1, &mut vao);
                gl.BindVertexArray(vao);
            }

            let pos_attrib = gl.GetAttribLocation(program, b"position\0".as_ptr() as *const _);
            let color_attrib = gl.GetAttribLocation(program, b"tex_coords\0".as_ptr() as *const _);
            info!("pos_attrib: {}", pos_attrib);
            info!("color_attrib: {}", color_attrib);
            println!("pos_attrib: {}", pos_attrib);
            println!("color_attrib: {}", color_attrib);
            gl.VertexAttribPointer(
                pos_attrib as gl::types::GLuint,
                2,
                gl::FLOAT,
                0,
                5 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                std::ptr::null(),
            );
            gl.VertexAttribPointer(
                color_attrib as gl::types::GLuint,
                3,
                gl::FLOAT,
                0,
                5 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                (2 * std::mem::size_of::<f32>()) as *const () as *const _,
            );
            gl.EnableVertexAttribArray(pos_attrib as gl::types::GLuint);
            gl.EnableVertexAttribArray(color_attrib as gl::types::GLuint);

            tex_location = gl.GetUniformLocation(program, b"tex\0".as_ptr() as *const _);
            info!("tex_location: {}", tex_location);
            println!("tex_location: {}", tex_location);

            gl.Uniform1i(tex_location, 0);
            gl.GenTextures(1, &mut tex_id);
            gl.BindTexture(gl::TEXTURE_2D, tex_id);
            gl.TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RED as i32,
                width as i32,
                height as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                std::ptr::null() as *const libc::c_void,
            );
            gl.TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_S,
                gl::REPEAT.try_into().unwrap(),
            );
            gl.TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_T,
                gl::REPEAT.try_into().unwrap(),
            );
            gl.TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR.try_into().unwrap(),
            );
            gl.TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                gl::LINEAR.try_into().unwrap(),
            );

        }
        info!("end creation PureGLRenderer");
        println!("end creation PureGLRenderer");
        PureGlRenderer { gl, tex_id: Some(tex_id), width, height, tex_location: Some(tex_location) }
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

            self.gl.Uniform1i(self.tex_location.unwrap(), 0);
            self.gl.BindTexture(gl::TEXTURE_2D, self.tex_id.unwrap());
            let mut data: Vec<Vec<u8>> = Vec::new();
            for i in 0..self.width {
                let mut v: Vec<u8> = Vec::new();
                for j in 0..self.height {
                    v.push((i * j % 255) as u8);
                }
                data.push(v);
            }
            self.gl.TexSubImage2D(
                gl::TEXTURE_2D,
                0,
                0,
                0,
                self.width as i32,
                self.height as i32,
                0,
                gl::RED,
                data.as_slice().as_ptr() as *const libc::c_void,
            );

            self.gl.DrawArrays(gl::TRIANGLE_STRIP, 0, 3);
        }
        info!("did draw!");
        Ok(())
    }
}


#[rustfmt::skip]
static VERTEX_DATA: [f32; 15] = [
    -0.5, -0.5,  1.0,  0.0,  0.0,
    0.0,  0.5,  0.0,  1.0,  0.0,
    0.5, -0.5,  0.0,  0.0,  1.0,
];

pub const VS_SRC: &[u8] = b"#version 300 es
in vec2 position;
in vec2 tex_coords;
out vec2 v_tex_coords;
void main() {
    v_tex_coords = tex_coords;
    gl_Position = vec4(position, 0.0, 1.0);
}
\0";

pub const FS_SRC: &[u8] = b"#version 300 es
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
}
\0";


pub fn shader_info(
    gl: &gl::Gl,
    shader: gl::types::GLuint,
    info_type: gl::types::GLenum,
    context: &str,
)  {
    let mut r = gl::FALSE as gl::types::GLint;
    unsafe { gl.GetShaderiv(shader, info_type, &mut r) };
    if r != gl::TRUE as gl::types::GLint {
        let mut s = [0u8; 1024].to_vec();
        let mut len = 0;
        unsafe {
            gl.GetShaderInfoLog(
                shader,
                s.len() as gl::types::GLint,
                &mut len,
                s.as_mut_ptr() as *mut gl::types::GLchar,
            );
        }
        let err = String::from_utf8(s[0..len as usize].to_vec()).unwrap();
        info!("shader error: {}", err);
        println!("shader error: {}", err);

    }
}