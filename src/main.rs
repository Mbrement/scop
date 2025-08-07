mod winsdl;
use sdl2::{event, video, EventPump, Sdl};

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
use gl::{self, types::GLint};
use imagine::{png, Bitmap};

type Vertex = [f32; 3 + 2]; // pos (3)+ tex (2)
type TriIndexes = [u32; 3];

use math;

//  it is a simple vertex structure with position and texture coordinates
const VERTICES: [Vertex; 4] = [
    // top right
    [0.5, 0.5, 0.0, 1.0, 1.0],
    // bottom right
    [0.5, -0.5, 0.0, 1.0, 0.0],
    // bottom left
    [-0.5, -0.5, 0.0, 0.0, 0.0],
    // top left
    [-0.5, 0.5, 0.0, 0.0, 1.0],
];

const INDICES: [TriIndexes; 2] = [[0, 1, 3], [1, 2, 3]];

fn main() {
    // let mut ticks_nb = 0.0;
    let logo = {
        let mut f = std::fs::File::open("./src/logo.png").unwrap();
        let mut bytes = vec![];
        std::io::Read::read_to_end(&mut f, &mut bytes).unwrap();
        let bitmap: Bitmap = imagine::png::png_try_bitmap_rgba(&bytes, false).unwrap();
        bitmap
    }; //TODO : load the texture without external lib HALP

    let garry = {
        let mut f = std::fs::File::open("./src/garry.png").unwrap();
        let mut bytes = vec![];
        std::io::Read::read_to_end(&mut f, &mut bytes).unwrap();
        let bitmap: Bitmap = imagine::png::png_try_bitmap_rgba(&bytes, false).unwrap();
        bitmap
    }; //TODO : load the texture without external lib HALP

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
    let ebo = helper::Buffer::new().expect("Couldn't make the element buffer.");
    ebo.bind(helper::BufferType::ElementArray);
    helper::Buffer::buffer_data(
        helper::BufferType::ElementArray,
        bytemuck::cast_slice(&INDICES),
        gl::STATIC_DRAW,
    );

    let shader_program_uniform = helper::ShaderProgram::from_vert_frag(
        shader::VERT_SHADER_UNIFORM,
        shader::FRAG_SHADER_UNIFORM,
    )
    .unwrap();

    let mut logo_texture = 0;
    unsafe {
        gl::GenTextures(1, &mut logo_texture);
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, logo_texture);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as GLint,
            logo.width as i32,
            logo.height as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            logo.pixels.as_ptr() as *const _,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }

    let mut garry_texture = 0;
    unsafe {
        gl::GenTextures(1, &mut garry_texture);
        gl::ActiveTexture(gl::TEXTURE1);
        gl::BindTexture(gl::TEXTURE_2D, garry_texture);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as GLint,
            garry.width as i32,
            garry.height as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            garry.pixels.as_ptr() as *const _,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }

    let shader_program =
        helper::ShaderProgram::from_vert_frag(shader::VERT_SHADER, shader::FRAG_SHADER).unwrap();
    shader_program.use_program();
    let mut uni_color_loc = -1;

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
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            size_of::<Vertex>().try_into().unwrap(),
            size_of::<[f32; 3]>() as *const _,
        );
        gl::EnableVertexAttribArray(1);

        let logo_name = "logo_texture\0".as_ptr().cast();
        gl::Uniform1i(gl::GetUniformLocation(shader_program.0, logo_name), 0);
        let garris_name = "garris_texture\0".as_ptr().cast();
        gl::Uniform1i(gl::GetUniformLocation(shader_program.0, garris_name), 1);
        let time_name = "time\0".as_ptr().cast();
        gl::Uniform1f(gl::GetUniformLocation(shader_program.0, time_name), 0.0);
    }
	let projection = ultraviolet::projection::perspective_gl(
    45.0_f32.to_radians(),
    (config::WindowConfig::WIDTH as f32) / (config::WindowConfig::HEIGHT as f32),
    0.1,
    100.0,
  );

	let model_loc = unsafe {
	let name = "model\0".as_ptr().cast();
		gl::GetUniformLocation(shader_program.0, name)
	};
	let view_loc = unsafe {
		let name = "view\0".as_ptr().cast();
		gl::GetUniformLocation(shader_program.0, name)
	};
	let projection_loc = unsafe {
		let name = "projection\0".as_ptr().cast();
		gl::GetUniformLocation(shader_program.0, name)
	};
	unsafe {
    gl::UniformMatrix4fv(projection_loc, 1, gl::FALSE, projection.as_ptr())
  };

    let view = math::Mat4::from_);
  unsafe { gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, view.as_ptr()) };
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
                    if running_engine & 1 != 0 {
                        shader_program.use_program();
                        running_engine -= 1;
                        break;
                    }
                    uni_color_loc = unsafe {
                        gl::GetUniformLocation(
                            shader_program_uniform.0,
                            b"uni_color\0".as_ptr().cast(),
                        )
                    };
                    if uni_color_loc < 0 {
                        panic!("\x1b[93mFailed to get uniform location for 'uni_color'\x1b[0m");
                    }
                    shader_program_uniform.use_program();
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

        // Get the location of the uniform variable in the shader program

        let ticks_nb = window.sdl.timer().unwrap().ticks() as f32 / 1000.0;
        let color_sin_value = f32::sin(ticks_nb / 1000.0);
        helper::clear_color(0., 0.33 + color_sin_value, 0.66 + color_sin_value, 1.0);
        helper::clear(gl::COLOR_BUFFER_BIT);

        unsafe {
            // gl::DrawArrays(gl::TRIANGLES, 0, 3);
            // gl::UseProgram(shader_program.0);
            if uni_color_loc != -1 {
                gl::Uniform4f(uni_color_loc, 0.1, color_sin_value, 0.1, 1.0);
                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const _);
            } else {
                let transform = math::Mat4::from_rotation_z(ticks_nb);
                shader_program.use_program();
                let transform_name = "transform\0".as_ptr().cast();
                let transform_loc: i32 = gl::GetUniformLocation(shader_program.0, transform_name);
                // if transform_loc < 0 {
                //     // println!("\x1b[93mFailed to get uniform location for 'transform'\x1b[0m");
                //     // --> actually happen, but why
                // }
                gl::UniformMatrix4fv(
                    transform_loc,
                    1,
                    gl::FALSE,
                    // transform.to_cols_array().as_ptr(),
                    transform.to_cols_array().as_ptr(),
                );
                let time_loc = gl::GetUniformLocation(shader_program.0, "time\0".as_ptr().cast());
                gl::Uniform1f(
                    time_loc,
                    window.sdl.timer().unwrap().ticks() as f32 / 2000.0 % 1.0,
                );
                // ticks_nb += 1.0;
                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const _);
            }
        }
        // println!("Ticks true: {}", window.sdl.timer().unwrap().ticks());
        // println!("Ticks: {}", ticks_nb);
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
