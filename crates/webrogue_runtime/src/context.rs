pub struct Context {
    // pub memory: *mut wasi_common::wiggle::GuestMemory<'a>,
    pub wasi: wasi_common::WasiCtx,
    pub memory_factory: Box<dyn MemoryFactory>,
}

impl Context {
    pub fn new(memory_factory: Box<dyn MemoryFactory>) -> Self {
        let wasi = wasi_common::WasiCtx::new(
            crate::random::random_ctx(),
            crate::clocks::clocks_ctx(),
            crate::sched::sched_ctx(),
            wasi_common::Table::new(),
        );
        wasi.set_stdout(Box::new(wasi_common::pipe::WritePipe::new(
            std::io::stdout(),
        )));
        wasi.set_stderr(Box::new(wasi_common::pipe::WritePipe::new(
            std::io::stderr(),
        )));
        Self {
            memory_factory,
            wasi,
        }
    }
}

pub trait MemoryFactory {
    fn make_memory(&self) -> crate::GuestMemory;
}
