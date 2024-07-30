pub use crate::context::Context;

pub fn glViewport(
    memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    context: &mut Context,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) {
    unsafe { crate::ffi::glViewport(x, y, width, height) }
}

pub fn glClearColor(
    memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    context: &mut Context,
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
) {
    unsafe { crate::ffi::glClearColor(red, green, blue, alpha) }
}

pub fn glClear(
    memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    context: &mut Context,
    mask: u32,
) {
    unsafe { crate::ffi::glClear(mask) }
}

pub fn present(
    memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    context: &mut Context,
) {
    context.window.gl_swap_window();
}
