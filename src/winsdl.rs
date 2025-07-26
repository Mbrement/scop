use sdl2::{
    video::{gl_attr, Window},
    EventPump, Sdl,
};

pub struct WindowSDL {
    pub sdl: Sdl,
    pub window: Window,
    pub gl_context: sdl2::video::GLContext,
    pub gl: (),
    pub event_pump: EventPump,
}

impl WindowSDL {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let sdl = sdl2::init().expect("Failed to initialize SDL");
        let video_subsystem = sdl.video().expect("Failed to get video subsystem");
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 6);

        let window = video_subsystem
            .window(title, width, height)
            .position_centered()
            .opengl()
            .build()
            .expect("Failed to create window");
        let gl_context = window.gl_create_context().unwrap();
        let gl = gl::load_with(|s| {
            video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
        });

        let event_pump = sdl.event_pump().expect("Failed to get event pump");

        WindowSDL {
            sdl,
            window,
            gl_context,
            gl,
            event_pump,
        }
    }
}
