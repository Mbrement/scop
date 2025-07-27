mod winsdl;
use sdl2::{event, video, EventPump, Sdl};

use cgmath::num_traits::ToPrimitive;
mod config;
mod helper;
mod shader;
use chrono::*;
use core::{
    convert::{TryFrom, TryInto},
    f32,
    mem::{size_of, size_of_val},
    time,
};
use gl::{types::GLuint, *};

use crate::helper::ShaderType;

type Vertex = [f32; 3];
type TriIndexes = [u32; 3];

const VERTICES: [Vertex; 4] = [
    [0.5, 0.5, 0.0],
    [0.5, -0.5, 0.0],
    [-0.5, -0.5, 0.0],
    [-0.5, 0.5, 0.0],
];
const COLORS: [Vertex; 3] = [[0.5, 0.5, 0.0], [0.0, 0.5, 0.5], [0.0, 0.5, 0.0]];
const INDICES: [TriIndexes; 2] = [[0, 1, 3], [1, 2, 3]];

fn main() {
    let start = chrono::DateTime::timestamp_millis(&self::Utc::now());
    let mut running_engine = 0; // flag to control shader program usage
    let mut window = winsdl::WindowSDL::new(
        config::WindowConfig::NAME,
        config::WindowConfig::WIDTH,
        config::WindowConfig::HEIGHT,
    );
    let vao = helper::VertexArray::new().expect("Failed to create Vertex Array Object");
    vao.bind();
    let vbo = helper::Buffer::new().expect("Couldn't make a VBO");
    vbo.bind(helper::BufferType::Array);

    let shader_program_uniform = helper::ShaderProgram::from_vert_frag(
        shader::VERT_SHADER_UNIFORM,
        shader::FRAG_SHADER_UNIFORM,
    )
    .unwrap();

    let shader_program =
        helper::ShaderProgram::from_vert_frag(shader::VERT_SHADER, shader::FRAG_SHADER).unwrap();
    shader_program.use_program();
    let mut uni_color_loc = -1;

    'running: loop {
        for event in window.event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => {
                    break 'running;
                }
                sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::W),
                    ..
                } => {
                    helper::polygon_mode(helper::PolygonMode::Line);
                }
                sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::F),
                    ..
                } => {
                    helper::polygon_mode(helper::PolygonMode::Fill);
                }
                sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::P),
                    ..
                } => {
                    helper::polygon_mode(helper::PolygonMode::Point);
                }
                sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::LCTRL),
                    ..
                } => {
                    if running_engine & 1 == 0 {
                        shader_program.use_program();
                        running_engine -= 1;
                        break;
                    }
                    shader_program_uniform.use_program();
                    uni_color_loc = unsafe {
                        gl::GetUniformLocation(
                            shader_program_uniform.0,
                            b"uni_color\0".as_ptr().cast(),
                        )
                    };
                    if uni_color_loc < 0 {
                        panic!("\x1b[93mFailed to get uniform location for 'uni_color'\x1b[0m");
                    }
                    running_engine += 1;
                }
                sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::LALT),
                    ..
                } => {
					if running_engine & 2 == 0 {
						running_engine -= 2;
					} else {
						running_engine += 2;
					}
				}
                _ => {}
            }
        }

        helper::Buffer::buffer_data(
            helper::BufferType::Array,
            bytemuck::cast_slice(&VERTICES),
            gl::STATIC_DRAW,
        );
        let ebo = helper::Buffer::new().expect("Couldn't make the element buffer.");
        ebo.bind(helper::BufferType::ElementArray);
        helper::Buffer::buffer_data(
            helper::BufferType::ElementArray,
            bytemuck::cast_slice(&INDICES),
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
        }

        // Get the location of the uniform variable in the shader program

        // let d_time = chrono::DateTime::timestamp_millis(&self::Utc::now()) - start;
        let green = f32::sin(window.sdl.timer().unwrap().ticks() as f32 / 1000.0_f32);
        // let color_timed = d_time.to_f32().expect("Failed to convert time to f32") % 1000.0 / 900.0;
        helper::clear_color(green, green + 0.33, green + 0.66, 1.0);
        helper::clear(gl::COLOR_BUFFER_BIT);
        let green = (f32::sin(window.sdl.timer().unwrap().ticks() as f32 / 1000.0_f32) / 2.0) + 0.5;
        unsafe {
            // gl::DrawArrays(gl::TRIANGLES, 0, 3);
            // gl::UseProgram(shader_program.0);
            if uni_color_loc != -1 {
                gl::Uniform4f(uni_color_loc, 0.1, green, 0.1, 1.0);
                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const _);
            } else {
                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const _);
            }
        }
        window.window.gl_swap_window();
    }
    shader_program.delete();
    shader_program_uniform.delete();
    // Print the time taken to run the program
    println!(
        "\x1b[92mExiting normaly after {} ms\x1b[0m",
        chrono::DateTime::timestamp_millis(&self::Utc::now()) - start
    );
}
