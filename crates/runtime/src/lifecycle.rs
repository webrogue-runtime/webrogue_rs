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
        archive_path: std::path::PathBuf,
    ) -> Result<()> {
        let runtime = backend.make_runtime();
        let bytecode = wrapp::read_wasm(archive_path)?;
        runtime.run(wasi, bytecode)?;
        Ok(())
    }
}
