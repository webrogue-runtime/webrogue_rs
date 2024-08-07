use wasi_common::snapshots::preview_1::wasi_snapshot_preview1::WasiSnapshotPreview1;
pub use wasi_common::WasiCtx as Context;
use wiggle::GuestPtr;

pub fn args_get(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut wasi_common::WasiCtx,
    _a: u32,
    _b: u32,
) -> u32 {
    0
}

pub fn args_sizes_get(
    memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    context: &mut wasi_common::WasiCtx,
    a: u32,
    b: u32,
) -> u32 {
    let mut memory = memory_factory.make_memory();
    let args = context.args_sizes_get(&mut memory);
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
pub fn proc_exit(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut wasi_common::WasiCtx,
    _code: u32,
) -> () {
    todo!()
}

pub fn fd_close(
    memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    context: &mut wasi_common::WasiCtx,
    fd: u32,
) -> u32 {
    let mut memory = memory_factory.make_memory();
    match futures::executor::block_on(context.fd_close(&mut memory, fd.into())) {
        Ok(_) => 0,
        Err(_) => 1,
    }
}

pub fn fd_fdstat_get(
    memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    context: &mut wasi_common::WasiCtx,
    fd: u32,
    out: u32,
) -> u32 {
    let mut memory = memory_factory.make_memory();

    match futures::executor::block_on(context.fd_fdstat_get(&mut memory, fd.into())) {
        Ok(fdstat) => {
            match memory.write(wiggle::GuestPtr::<_>::new(out), fdstat) {
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
pub fn fd_seek(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut wasi_common::WasiCtx,
    _a: u32,
    _b: u64,
    _c: u32,
    _d: u32,
) -> u32 {
    todo!()
}
pub fn fd_write(
    memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    context: &mut wasi_common::WasiCtx,
    fd: u32,
    wasi_iovs: u32,
    iovs_len: u32,
    nwritten_ptr: u32,
) -> u32 {
    let mut memory = memory_factory.make_memory();

    match futures::executor::block_on(context.fd_write(
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

pub fn fd_fdstat_set_flags(
    memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut wasi_common::WasiCtx,
    _a: u32,
    _b: u32,
) -> u32 {
    let _memory = memory_factory.make_memory();
    todo!()
}
pub fn fd_prestat_get(
    memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    context: &mut wasi_common::WasiCtx,
    fd: u32,
    buf: u32,
) -> u32 {
    let mut memory = memory_factory.make_memory();

    match futures::executor::block_on(context.fd_prestat_get(
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
        Err(_) => {
            return 8;
        }
    };
}
pub fn fd_prestat_dir_name(
    memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    context: &mut wasi_common::WasiCtx,
    fd: u32,
    path: u32,
    path_len: u32,
) -> u32 {
    let mut memory: wiggle::GuestMemory = memory_factory.make_memory();

    let future =
        context.fd_prestat_dir_name(&mut memory, fd.into(), GuestPtr::<u8>::new(path), path_len);

    match futures::executor::block_on(future) {
        Err(_) => 1,
        Ok(()) => 0,
    }
}
pub fn fd_read(
    memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    context: &mut wasi_common::WasiCtx,
    fd: u32,
    wasi_iovs: u32,
    iovs_len: u32,
    nread: u32,
) -> u32 {
    let mut memory = memory_factory.make_memory();
    let future = context.fd_read(
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
    memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    context: &mut wasi_common::WasiCtx,
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
    let mut memory = memory_factory.make_memory();

    let future = context.path_open(
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

pub fn poll_oneoff(
    memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    context: &mut wasi_common::WasiCtx,
    subscription_in_ptr: u32,
    out_ptr: u32,
    nsubscriptions: u32,
    nevents_ptr: u32,
) -> u32 {
    let mut memory = memory_factory.make_memory();

    let future = context.poll_oneoff(
        &mut memory,
        GuestPtr::<wasi_common::snapshots::preview_1::types::Subscription>::new(
            subscription_in_ptr,
        ),
        GuestPtr::<wasi_common::snapshots::preview_1::types::Event>::new(out_ptr),
        nsubscriptions,
    );

    match futures::executor::block_on(future) {
        Err(_) => 1,
        Ok(result) => {
            memory
                .write(GuestPtr::<_>::new(nevents_ptr), result)
                .unwrap();
            0
        }
    }
}
