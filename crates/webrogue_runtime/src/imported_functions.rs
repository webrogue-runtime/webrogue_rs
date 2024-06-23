use crate::context::Context;
use wasi_common::snapshots::preview_1::wasi_snapshot_preview1::WasiSnapshotPreview1;

pub fn wr_imported_func_1(_context: &mut Context, a: u32) {
    println!("imported_func_1({})", a)
}

pub fn wr_imported_func_2(_context: &mut Context, a: u32) -> u32 {
    println!("imported_func_2({})", a);
    a + 1
}

pub fn wr_imported_func_3(context: &mut Context, a: u32, b: u32) {
    // println!("memory_size({})", context.memory.size());
    println!("imported_func_3({}, {})", a, b);
    let mut memory = context.memory_factory.make_memory();
    let a = wiggle::GuestPtr::<u16>::new(a);
    let dat = memory.read(a);
    memory.write(a, 48);
}

pub fn wasi_snapshot_preview1_args_get(_context: &mut Context, a: u32, b: u32) -> u32 {
    0
}

pub fn wasi_snapshot_preview1_args_sizes_get(context: &mut Context, a: u32, b: u32) -> u32 {
    // let mut bytes = vec![0 as u8; 64 * 1024 * 1];
    // let mut memory = wiggle::GuestMemory::Unshared(bytes.as_mut_slice());
    let mut memory = context.memory_factory.make_memory();
    let args = context.wasi.args_sizes_get(&mut memory);
    match futures::executor::block_on(args) {
        Ok(args) => {
            match memory.write(wiggle::GuestPtr::<u32>::new(a), args.0) {
                Err(_) => {
                    return 1;
                }
                Ok(_) => {}
            };
            memory.write(wiggle::GuestPtr::<u32>::new(b), args.1);
            return 0;
        }
        Err(_) => {
            return 1;
        }
    };
}
pub fn wasi_snapshot_preview1_proc_exit(_context: &mut Context, a: u32) -> () {}
