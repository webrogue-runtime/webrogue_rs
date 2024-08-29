pub mod backend;

pub struct Context {
    pub system_maker: Box<dyn Fn() -> Box<dyn backend::System>>,
    pub system: Option<Box<dyn backend::System>>,
    pub window: Option<Box<dyn backend::Window>>,
}

// for ios DispatchQueue
// extern "C" {
//     fn webrogueRunOnMainThread<'a>(
//         f: extern "C" fn(userdata: *mut Box<dyn FnMut()>),
//         userdata: *mut Box<(dyn FnMut() + 'a)>,
//     );
// }

// for ios DispatchQueue
// extern "C" fn box_runner(userdata_ptr: *mut Box<dyn FnMut()>) {
//     // maybe unsafe cz it is actually FnOnce
//     unsafe {
//         // let f2 = userdata.as_ref().unwrap();
//         // let f3 = f2.as_ref();
//         // f3();
//         (*userdata_ptr)()
//     };
// }

// for ios DispatchQueue
#[inline]
fn run_on_main_thread<T>(mut f: impl FnMut() -> T) -> T {
    return f();
    // let mut result: MaybeUninit<T> = MaybeUninit::uninit();
    // let result_ptr: *mut MaybeUninit<T> = &mut result;
    // let mut userdata: Box<dyn FnMut()> = Box::new(|| unsafe {
    //     result_ptr.as_mut().unwrap().write(f());
    // });
    // let userdata_ptr: *mut Box<dyn FnMut()> = &mut userdata;
    // unsafe { webrogueRunOnMainThread(box_runner, userdata_ptr) };
    // drop(userdata);
    // return unsafe { result.assume_init() };
}

impl Context {
    pub fn new(system_maker: Box<dyn Fn() -> Box<dyn backend::System>>) -> Self {
        Context {
            system_maker,
            system: None,
            window: None,
        }
    }
}

pub fn make_window(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
) {
    run_on_main_thread(|| {
        if _context.system.is_none() {
            _context.system = Some((_context.system_maker)())
        }
        let system = _context.system.as_ref().unwrap();
        if _context.window.is_none() {
            _context.window = Some(system.make_window())
        }
    });
}

pub fn present(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
) {
    run_on_main_thread(|| {
        _context.window.as_mut().inspect(|window| {
            window.present();
        });
    })
}

pub fn get_window_width(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
) -> u32 {
    run_on_main_thread(|| {
        _context
            .window
            .as_ref()
            .and_then(|window| Some(window.get_size().0))
            .unwrap_or_default()
    })
}

pub fn get_window_height(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
) -> u32 {
    run_on_main_thread(|| {
        _context
            .window
            .as_ref()
            .and_then(|window| Some(window.get_size().1))
            .unwrap_or_default()
    })
}
