use beryllium::{
    events::Event,
    init::InitFlags,
    video::{CreateWinArgs, GlContextFlags, GlProfile, GlSwapInterval},
    *,
};
use cgmath::num_traits::ToPrimitive;
mod config;
mod helper;
use chrono::*;
use core::{
    convert::{TryFrom, TryInto},
    f32,
    mem::{size_of, size_of_val},
    time,
};
use gl::{types::GLuint, *};

type Vertex = [f32; 3];
const VERTICES: [Vertex; 3] = [[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0]];
const COLORS: [Vertex; 3] = [[0.5, 0.5, 0.0], [0.0, 0.5, 0.5], [0.0, 0.5, 0.0]];

fn main() {
    let start = chrono::DateTime::timestamp_millis(&self::Utc::now());

    let sdl = Sdl::init(init::InitFlags::EVERYTHING);
    sdl.set_gl_context_major_version(4).unwrap();
    sdl.set_gl_context_minor_version(6).unwrap();
    sdl.set_gl_profile(video::GlProfile::Core).unwrap();

    let win_args = video::CreateWinArgs {
        title: config::WindowConfig::NAME,
        width: config::WindowConfig::WIDTH,
        height: config::WindowConfig::HEIGHT,
        allow_high_dpi: true,
        borderless: false,
        resizable: false,
    };

    let win = sdl
        .create_gl_window(win_args)
        .expect("couldn't make a window and context");
    'main_loop: loop {
        // handle events this frame
        while let Some(event) = sdl.poll_events() {
            match event {
                (events::Event::Quit, _) => break 'main_loop,
                (events::Event::Key { keycode, .. }, _) => {
                    if keycode == events::SDLK_ESCAPE {
                        break 'main_loop;
                    }
                }
                _ => (),
            }
        }

        unsafe {
            gl::load_with(|s| {
                std::ffi::CString::new(s)
                    .ok()
                    .map_or(std::ptr::null(), |cstr| {
                        win.get_proc_address(cstr.as_ptr() as *const u8)
                    })
            });
        }
        let vao = helper::VertexArray::new().expect("Failed to create Vertex Array Object");
		vao.bind();
		let vbo = helper::Buffer::new().expect("Couldn't make a VBO");
			vbo.bind(helper::BufferType::Array);
			helper::Buffer::buffer_data(
			helper::BufferType::Array,
			bytemuck::cast_slice(&VERTICES),
			gl::STATIC_DRAW,
			);
        unsafe {
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                size_of::<Vertex>().try_into().unwrap(),
                0 as *const _,
            );
            gl::EnableVertexAttribArray(0);
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            assert_ne!(vertex_shader, 0);
            const VERT_SHADER: &str = r#"#version 330 core
  layout (location = 0) in vec3 pos;
  void main() {
    gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
  }
"#;
            gl::ShaderSource(
                vertex_shader,
                1,
                &(VERT_SHADER.as_bytes().as_ptr().cast()),
                &(VERT_SHADER.len().try_into().unwrap()),
            );
            gl::CompileShader(vertex_shader);
            let mut success = 0;
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetShaderInfoLog(vertex_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
                panic!("Vertex Compile Error: {}", String::from_utf8_lossy(&v));
            }

            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            assert_ne!(fragment_shader, 0);
            const FRAG_SHADER: &str = r#"#version 330 core
  out vec4 final_color;
  void main() {
  float x = gl_FragCoord.x / 800.0;
  float y = gl_FragCoord.y / 600.0;
  float z = gl_FragCoord.z;
	final_color = vec4(x, y, z, 1.0);
  }
"#;
            gl::ShaderSource(
                fragment_shader,
                1,
                &(FRAG_SHADER.as_bytes().as_ptr().cast()),
                &(FRAG_SHADER.len().try_into().unwrap()),
            );
            gl::CompileShader(fragment_shader);
            let mut success = 0;
            gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetShaderInfoLog(fragment_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
                panic!("Fragment Compile Error: {}", String::from_utf8_lossy(&v));
            }
            let shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);
            let mut success = 0;
            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetProgramInfoLog(shader_program, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
                panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
            }
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
            let _ = win.set_swap_interval(GlSwapInterval::Vsync);
            helper::clear(gl::COLOR_BUFFER_BIT);
            gl::UseProgram(shader_program);
        }
        let d_time = chrono::DateTime::timestamp_millis(&self::Utc::now()) - start;
        let color_timed = d_time.to_f32().expect("Failed to convert time to f32") % 1000.0 / 900.0;
        helper::clear_color(color_timed, color_timed + 0.33, color_timed + 0.66, 1.0);
        helper::clear(gl::COLOR_BUFFER_BIT);
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl::DrawArrays(gl::TRIANGLES, 1, 3);
		}
        win.swap_window();
    }
}
