pub struct Context {
    pub window: Option<sdl2::video::Window>,
    pub gl_context: Option<sdl2::video::GLContext>,
    pub video_subsystem: Option<sdl2::VideoSubsystem>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            window: None,
            gl_context: None,
            video_subsystem: None,
        }
    }
}

pub fn make_window(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    video_subsystem.gl_attr().set_double_buffer(true);
    video_subsystem.gl_attr().set_context_profile(sdl2::video::GLProfile::GLES);
    video_subsystem.gl_attr().set_context_version(2, 0);
    let _ = video_subsystem.gl_load_library_default();
    let window = video_subsystem
        .window("webrogue", 800, 450)
        .opengl()
        .resizable()
        .build()
        .unwrap();
    let gl_context: sdl2::video::GLContext = window.gl_create_context().unwrap();
    _context.gl_context = Some(gl_context);
    _context.window = Some(window);
    _context.video_subsystem = Some(video_subsystem);
}

pub fn present(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
) {
    _context.window.as_mut().inspect(|window| {
        window.gl_swap_window();
    });
}

pub fn get_window_width(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
) -> u32 {
    _context
        .window
        .as_ref()
        .and_then(|window| Some(window.size().0))
        .unwrap_or_default()
}

pub fn get_window_height(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
) -> u32 {
    _context
        .window
        .as_ref()
        .and_then(|window| Some(window.size().1))
        .unwrap_or_default()
}
