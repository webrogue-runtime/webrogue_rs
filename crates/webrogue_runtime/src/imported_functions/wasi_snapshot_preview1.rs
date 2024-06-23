use crate::context::Context;
use wasi_common::snapshots::preview_1::wasi_snapshot_preview1::WasiSnapshotPreview1;
use wiggle::GuestPtr;

pub fn args_get(_context: &mut Context, a: u32, b: u32) -> u32 {
    0
}

pub fn args_sizes_get(context: &mut Context, a: u32, b: u32) -> u32 {
    let mut memory = context.memory_factory.make_memory();
    let args = context.wasi.args_sizes_get(&mut memory);
    match futures::executor::block_on(args) {
        Ok(args) => {
            match memory.write(wiggle::GuestPtr::new(a), args.0) {
                Err(_) => {
                    return 1;
                }
                Ok(_) => {}
            }
            match memory.write(wiggle::GuestPtr::new(b), args.1) {
                Err(_) => {
                    return 1;
                }
                Ok(_) => {}
            }
            return 0;
        }
        Err(_) => {
            return 1;
        }
    };
}
pub fn proc_exit(_context: &mut Context, _code: u32) -> () {
    todo!()
}

pub fn fd_close(context: &mut Context, fd: u32) -> u32 {
    let mut memory = context.memory_factory.make_memory();
    match futures::executor::block_on(context.wasi.fd_close(&mut memory, fd.into())) {
        Ok(_) => 0,
        Err(_) => 1,
    }
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

pub fn fd_fdstat_set_flags(context: &mut Context, a: u32, b: u32) -> u32 {
    let mut memory = context.memory_factory.make_memory();
    todo!()
}
pub fn fd_prestat_get(context: &mut Context, fd: u32, buf: u32) -> u32 {
    let mut memory = context.memory_factory.make_memory();

    match futures::executor::block_on(context.wasi.fd_prestat_get(
        &mut memory,
        wasi_common::snapshots::preview_1::types::Fd::from(fd),
    )) {
        Ok(prestat) => {
            match memory.write(wiggle::GuestPtr::<_>::new(buf), prestat) {
                Err(_) => {
                    return 1;
                }
                Ok(_) => {}
            };
            return 0;
        }
        Err(e) => {
            return 8;
        }
    };
}
pub fn fd_prestat_dir_name(context: &mut Context, fd: u32, path: u32, path_len: u32) -> u32 {
    let mut memory: wiggle::GuestMemory = context.memory_factory.make_memory();

    let future = context.wasi.fd_prestat_dir_name(
        &mut memory,
        fd.into(),
        GuestPtr::<u8>::new(path),
        path_len,
    );

    match futures::executor::block_on(future) {
        Err(_) => 1,
        Ok(()) => 0,
    }
}
pub fn fd_read(context: &mut Context, fd: u32, wasi_iovs: u32, iovs_len: u32, nread: u32) -> u32 {
    let mut memory = context.memory_factory.make_memory();
    let future = context.wasi.fd_read(
        &mut memory,
        fd.into(),
        wasi_common::snapshots::preview_1::types::IovecArray::new((wasi_iovs, iovs_len)),
    );
    match futures::executor::block_on(future) {
        Err(_) => 1,
        Ok(result) => {
            memory.write(GuestPtr::<_>::new(nread), result).unwrap();
            0
        }
    }
}
pub fn path_open(
    context: &mut Context,
    dirfd: u32,
    dirflags: u32,
    path: u32,
    path_len: u32,
    oflags: u32,
    fs_rights_base: u64,
    fs_rights_inheriting: u64,
    fs_flags: u32,
    fd: u32,
) -> u32 {
    let mut memory = context.memory_factory.make_memory();

    let future = context.wasi.path_open(
        &mut memory,
        dirfd.into(),
        wasi_common::snapshots::preview_1::types::Lookupflags::from_bits(dirflags).unwrap(),
        GuestPtr::<str>::new((path, path_len)),
        wasi_common::snapshots::preview_1::types::Oflags::from_bits(oflags as u16).unwrap(),
        wasi_common::snapshots::preview_1::types::Rights::from_bits(fs_rights_base).unwrap(),
        wasi_common::snapshots::preview_1::types::Rights::from_bits(fs_rights_inheriting).unwrap(),
        wasi_common::snapshots::preview_1::types::Fdflags::from_bits(fs_flags as u16).unwrap(),
    );

    match futures::executor::block_on(future) {
        Err(_) => 1,
        Ok(result) => {
            memory.write(GuestPtr::<_>::new(fd), result).unwrap();
            0
        }
    }
}
