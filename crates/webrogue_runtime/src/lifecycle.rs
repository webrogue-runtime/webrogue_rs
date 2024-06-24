use crate::backend::Backend;
pub use anyhow::Result;

pub struct Lifecycle {}

impl Lifecycle {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(
        &self,
        backend: impl Backend,
        wasi: wasi_common::WasiCtx,
        bytecode: Vec<u8>,
    ) -> Result<()> {
        let runtime = backend.make_runtime();
        runtime.run(wasi, bytecode)?;
        Ok(())
    }
}
