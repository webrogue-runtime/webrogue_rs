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

fn load_gl(video_subsystem: &sdl2::VideoSubsystem) {
    #[cfg(target_os = "macos")]
    {
        let result = (|| {
            let path = std::env::current_exe().ok()?;
            let path = path.parent()?;

            if path.join("libGLESv2.dylib").exists() && path.join("libEGL.dylib").exists() {
                std::env::set_var("SDL_VIDEO_EGL_DRIVER", path.join("libEGL.dylib"));
                std::env::set_var("SDL_VIDEO_GL_DRIVER", path.join("libGLESv2.dylib"));
                return Some(());
            } else {
                return None;
            }
        })();

        if result.is_some() {
            return;
        }
    }
    let result = video_subsystem.gl_load_library_default();
    if result.is_ok() {
        return;
    }

    panic!("failed to load opengl")
}

pub fn make_window(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    video_subsystem.gl_attr().set_double_buffer(true);
    video_subsystem
        .gl_attr()
        .set_context_profile(sdl2::video::GLProfile::GLES);
    video_subsystem.gl_attr().set_context_version(2, 0);
    load_gl(&video_subsystem);
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

    unsafe {
        let mut event: ::core::mem::MaybeUninit<sdl2::sys::SDL_Event> =
            ::core::mem::MaybeUninit::uninit();
        sdl2::sys::SDL_PollEvent(event.as_mut_ptr());
        if (*event.as_ptr()).type_ == sdl2::sys::SDL_EventType::SDL_QUIT as u32 {
            return;
        }
    }
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
