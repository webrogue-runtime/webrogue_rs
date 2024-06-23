use crate::context::Context;

pub fn imported_func_1(_context: &mut Context, a: u32) {
    println!("imported_func_1({})", a)
}

pub fn imported_func_2(_context: &mut Context, a: u32) -> u32 {
    println!("imported_func_2({})", a);
    a + 1
}

pub fn imported_func_3(context: &mut Context, a: u32, b: u32) {
    // println!("memory_size({})", context.memory.size());
    println!("imported_func_3({}, {})", a, b);
    let mut memory = context.memory_factory.make_memory();
    let a = wiggle::GuestPtr::<u16>::new(a);
    let dat = memory.read(a);
    memory.write(a, 48);
}
