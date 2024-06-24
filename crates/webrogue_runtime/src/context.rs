pub struct Context {
    pub wasi: wasi_common::WasiCtx,
    pub memory_factory: Box<dyn MemoryFactory>,
}

impl Context {
    pub fn new(memory_factory: Box<dyn MemoryFactory>, wasi: wasi_common::WasiCtx) -> Self {
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
