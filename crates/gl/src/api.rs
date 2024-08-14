#![allow(non_snake_case)]

pub use crate::context::Context;

webrogue_gl_gen_macro::make_rust_impl! {}

pub use crate::mainual_impl::*;

fn get_string(name: u32) -> Vec<u8> {
    let gl_str = unsafe { crate::ffi::glGetString(name) } as *const i8;
    if gl_str.is_null() { return Vec::new();}
    let owned_str = unsafe { std::ffi::CStr::from_ptr(gl_str) };
    owned_str.to_bytes_with_nul()
        .to_vec()
}

#[allow(non_snake_case)]
pub fn glGetStringData(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    name: u32,
    data_ptr: u32,
) {
    let s = get_string(name);
    let mut memory = _memory_factory.make_memory();
    let ptr = webrogue_runtime::wiggle::GuestPtr::<[u8]>::new((data_ptr, s.len() as u32));
    let _ = memory.copy_from_slice(s.as_slice(), ptr);
}
#[allow(non_snake_case)]
pub fn glGetStringLen(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    name: u32,
) -> u32 {
    get_string(name).len() as u32
}
