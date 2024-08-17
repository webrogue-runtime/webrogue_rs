pub use wasi_common::WasiCtx as Context;

// from https://github.com/wasmi-labs/wasmi/blob/main/crates/wasi/src/sync/snapshots/preview_1.rs

// Creates a dummy `RawWaker`. We can only create Wakers from `RawWaker`s
fn dummy_raw_waker() -> std::task::RawWaker {
    fn no_op(_: *const ()) {}
    //returns a new RawWaker by calling dummy_raw_waker again
    fn clone(_: *const ()) -> std::task::RawWaker {
        dummy_raw_waker()
    }
    // RawWakerVTable specifies the functions that should be called when the RawWaker is cloned, woken, or dropped.
    let vtable = &std::task::RawWakerVTable::new(clone, no_op, no_op, no_op);

    std::task::RawWaker::new(std::ptr::null::<()>(), vtable)
}

// Creates a dummy waker which does *nothing*, as the future itsef polls to ready at first poll
// A waker is needed to do any polling at all, as it is the primary constituent of the `Context` for polling
fn run_in_dummy_executor<F: std::future::Future>(f: F) -> anyhow::Result<F::Output> {
    let mut f = std::pin::Pin::from(Box::new(f));
    let waker = unsafe { std::task::Waker::from_raw(dummy_raw_waker()) };
    let mut cx = std::task::Context::from_waker(&waker);
    match f.as_mut().poll(&mut cx) {
        std::task::Poll::Ready(val) => Ok(val),
        std::task::Poll::Pending => anyhow::bail!("Cannot wait on pending future"),
    }
}

macro_rules! add_funcs {
    (
        $(
            fn $fname:ident ($( $arg:ident : $typ:ty ),* $(,)? ) -> $ret:tt
        );+ $(;)?
    ) => {
        $(
            pub fn $fname(
                    memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
                    context: &mut wasi_common::WasiCtx,
                    $($arg : $typ,)*
            ) -> $ret {
                run_in_dummy_executor(async {
                    let mut memory = memory_factory.make_memory();
                    match wasi_common::snapshots::preview_1::wasi_snapshot_preview1::$fname(context, &mut memory, $($arg,)*).await {
                        Ok(r) => <$ret>::from(r),
                        _ => panic!("{} returned error", stringify!($fname)),
                    }
                }).unwrap()
            }
        )*
    };
}

add_funcs! {
    fn args_get(argv: i32, argv_buf: i32) -> i32;
    fn args_sizes_get(offset0: i32, offset1: i32) -> i32;
    fn environ_get(environ: i32, environ_buf: i32) -> i32;
    fn environ_sizes_get(offset0: i32, offset1: i32) -> i32;
    fn clock_res_get(id: i32, offset0: i32) -> i32;
    fn clock_time_get(id: i32, precision: i64, offset0: i32) -> i32;
    fn fd_advise(fd: i32, offset: i64, len: i64, advice: i32) -> i32;
    fn fd_allocate(fd: i32, offset: i64, len: i64) -> i32;
    fn fd_close(fd: i32) -> i32;
    fn fd_datasync(fd: i32) -> i32;
    fn fd_fdstat_get(fd: i32, offset0: i32) -> i32;
    fn fd_fdstat_set_flags(fd: i32, flags: i32) -> i32;
    fn fd_fdstat_set_rights(fd: i32, fs_rights_base: i64, fs_rights_inheriting: i64) -> i32;
    fn fd_filestat_get(fd: i32, offset0: i32) -> i32;
    fn fd_filestat_set_size(fd: i32, size: i64) -> i32;
    fn fd_filestat_set_times(fd: i32, atim: i64, mtim: i64, fst_flags: i32) -> i32;
    fn fd_pread(fd: i32, iov_buf: i32, iov_buf_len: i32, offset: i64, offset0: i32) -> i32;
    fn fd_prestat_get(fd: i32, offset0: i32) -> i32;
    fn fd_prestat_dir_name(fd: i32, path: i32, path_len: i32) -> i32;
    fn fd_pwrite(fd: i32, ciov_buf: i32, ciov_buf_len: i32, offset: i64, offset0: i32) -> i32;
    fn fd_read(fd: i32, iov_buf: i32, iov_buf_len: i32, offset1: i32) -> i32;
    fn fd_readdir(fd: i32, buf: i32, buf_len: i32, cookie: i64, offset0: i32) -> i32;
    fn fd_renumber(fd: i32, to: i32) -> i32;
    fn fd_seek(fd: i32, offset: i64, whence: i32, offset0: i32) -> i32;
    fn fd_sync(fd: i32) -> i32;
    fn fd_tell(fd: i32, offset0: i32) -> i32;
    fn fd_write(fd: i32, ciov_buf: i32, ciov_buf_len: i32, offset0: i32) -> i32;
    fn path_create_directory(fd: i32, offset: i32, length: i32) -> i32;
    fn path_filestat_get(fd: i32, flags: i32, offset: i32, length: i32, offset0: i32) -> i32;
    fn path_filestat_set_times(
        fd: i32,
        flags: i32,
        offset: i32,
        length: i32,
        atim: i64,
        mtim: i64,
        fst_flags: i32,
    ) -> i32;
    fn path_link(
        old_fd: i32,
        old_flags: i32,
        old_offset: i32,
        old_length: i32,
        new_fd: i32,
        new_offset: i32,
        new_length: i32,
    ) -> i32;
    fn path_open(
        fd: i32,
        dirflags: i32,
        offset: i32,
        length: i32,
        oflags: i32,
        fs_rights_base: i64,
        fdflags: i64,
        fs_rights_inheriting: i32,
        offset0: i32,
    ) -> i32;
    fn path_readlink(
        fd: i32,
        offset: i32,
        length: i32,
        buf: i32,
        buf_len: i32,
        offset0: i32,
    ) -> i32;
    fn path_remove_directory(fd: i32, offset: i32, length: i32) -> i32;
    fn path_rename(
        fd: i32,
        old_offset: i32,
        old_length: i32,
        new_fd: i32,
        new_offset: i32,
        new_length: i32,
    ) -> i32;
    fn path_symlink(
        old_offset: i32,
        old_length: i32,
        fd: i32,
        new_offset: i32,
        new_length: i32,
    ) -> i32;
    fn path_unlink_file(fd: i32, offset: i32, length: i32) -> i32;
    fn poll_oneoff(in_: i32, out: i32, nsubscriptions: i32, offset0: i32) -> i32;
    fn proc_exit(rval: i32) -> ();
    fn proc_raise(sig: i32) -> i32;
    fn sched_yield() -> i32;
    fn random_get(buf: i32, buf_len: i32) -> i32;
    fn sock_accept(fd: i32, flags: i32, offset0: i32) -> i32;
    fn sock_recv(
        fd: i32,
        iov_buf: i32,
        iov_buf_len: i32,
        ri_flags: i32,
        offset0: i32,
        offset1: i32,
    ) -> i32;
    fn sock_send(fd: i32, ciov_buf: i32, ciov_buf_len: i32, si_flags: i32, offset0: i32) -> i32;
    fn sock_shutdown(fd: i32, how: i32) -> i32;
}
