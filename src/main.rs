// use beryllium::{
//     events::Event,
//     init::InitFlags,
//     video::{CreateWinArgs, GlContextFlags, GlProfile, GlSwapInterval},
//     *,
// };
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

    // let sdl = Sdl::init(init::InitFlags::EVERYTHING);
    // sdl.set_gl_context_major_version(4).unwrap();
    // sdl.set_gl_context_minor_version(6).unwrap();
    // sdl.set_gl_profile(video::GlProfile::Core).unwrap();
    //
    // let win_args = video::CreateWinArgs {
    // title: config::WindowConfig::NAME,
    // width: config::WindowConfig::WIDTH,
    // height: config::WindowConfig::HEIGHT,
    // allow_high_dpi: true,
    // borderless: false,
    // resizable: false,
    // };
    // let win = sdl
    // .create_gl_window(win_args)
    // .expect("couldn't make a window and context");
    let mut window = winsdl::WindowSDL::new(
        config::WindowConfig::NAME,
        config::WindowConfig::WIDTH,
        config::WindowConfig::HEIGHT,
    );
    let vao = helper::VertexArray::new().expect("Failed to create Vertex Array Object");
    vao.bind();
    let vbo = helper::Buffer::new().expect("Couldn't make a VBO");
    vbo.bind(helper::BufferType::Array);
	        let shader_program =
            helper::ShaderProgram::from_vert_frag(shader::VERT_SHADER, shader::FRAG_SHADER)
                .unwrap();
        shader_program.use_program();

    'running: loop {
        for event in window.event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => {
                    // helper::ShaderProgram::delete(shader_program);
                    break 'running;
                }
                sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => {
					// helper::ShaderProgram::delete(shader_program);
					break 'running
				},
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

        let d_time = chrono::DateTime::timestamp_millis(&self::Utc::now()) - start;
        let color_timed = d_time.to_f32().expect("Failed to convert time to f32") % 1000.0 / 900.0;
        helper::clear_color(color_timed, color_timed + 0.33, color_timed + 0.66, 1.0);
        helper::clear(gl::COLOR_BUFFER_BIT);
        unsafe {
            // gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const _);
        }
        window.window.gl_swap_window();
    }

    // Print the time taken to run the program
    println!(
        "Exiting after {} ms",
        chrono::DateTime::timestamp_millis(&self::Utc::now()) - start
    );
}

// mod winsdl;

// fn main(){

// 	let mut window = winsdl::WindowSDL::new("My Window", 800, 600);
// 	'running: loop{
// 		for event in window.event_pump.poll_iter() {
// 			match event {
// 				sdl2::event::Event::Quit { .. } => return,
// 				sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => return,
// 				_ => {}
// 			}
// 		}

// 		window.window.gl_swap_window();
// 		std::thread::sleep(std::time::Duration::from_millis(16)); // Roughly 60 FPS
// 	}
