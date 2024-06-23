use crate::context::Context;
use wasi_common::snapshots::preview_1::wasi_snapshot_preview1::WasiSnapshotPreview1;

pub fn args_get(_context: &mut Context, a: u32, b: u32) -> u32 {
    0
}

pub fn args_sizes_get(context: &mut Context, a: u32, b: u32) -> u32 {
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
pub fn proc_exit(_context: &mut Context, a: u32) -> () {
    todo!()
}

pub fn fd_close(_context: &mut Context, a: u32) -> u32 {
    todo!()
}

pub fn fd_fdstat_get(context: &mut Context, fd: u32, out: u32) -> u32 {
    let mut memory = context.memory_factory.make_memory();

    match futures::executor::block_on(context.wasi.fd_fdstat_get(&mut memory, fd.into())) {
        Ok(fdstat) => {
            match memory.write(
                wiggle::GuestPtr::<wasi_common::snapshots::preview_1::types::Fdstat>::new(out),
                fdstat,
            ) {
                Err(_) => {
                    return 1;
                }
                Ok(_) => {}
            };
            return 0;
        }
        Err(_) => {
            return 1;
        }
    };
}
pub fn fd_seek(_context: &mut Context, a: u32, b: u64, c: u32, d: u32) -> u32 {
    todo!()
}
pub fn fd_write(
    context: &mut Context,
    fd: u32,
    wasi_iovs: u32,
    iovs_len: u32,
    nwritten_ptr: u32,
) -> u32 {
    let mut memory = context.memory_factory.make_memory();

    match futures::executor::block_on(context.wasi.fd_write(
        &mut memory,
        fd.into(),
        wasi_common::snapshots::preview_1::types::CiovecArray::new((wasi_iovs, iovs_len)),
    )) {
        Ok(nwritten) => {
            match memory.write(wiggle::GuestPtr::<u32>::new(nwritten_ptr), nwritten) {
                Err(_) => {
                    return 1;
                }
                Ok(_) => {}
            };
            return 0;
        }
        Err(_) => {
            return 1;
        }
    };
}
