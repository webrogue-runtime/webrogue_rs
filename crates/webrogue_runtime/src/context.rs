pub struct Context {
    // pub memory: *mut wasi_common::wiggle::GuestMemory<'a>,
    pub wasi: wasi_common::WasiCtx,
    pub memory_factory: Box<dyn MemoryFactory>,
}

impl Context {
    pub fn new(memory_factory: Box<dyn MemoryFactory>) -> Self {
        Self {
            memory_factory,
            wasi: wasi_common::WasiCtx::new(
                wasi_common::sync::random_ctx(),
                wasi_common::sync::clocks_ctx(),
                wasi_common::sync::sched_ctx(),
                wasi_common::Table::new(),
            ),
        }
    }
}

pub trait MemoryFactory {
    fn make_memory(&self) -> crate::GuestMemory;
}
