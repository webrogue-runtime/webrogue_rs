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
        archive_reader: wrapp::Reader,
    ) -> Result<()> {
        let mut archive_reader = archive_reader;
        let runtime = backend.make_runtime();
        runtime.run(wasi, archive_reader.read_wasm()?)?;
        Ok(())
    }
}
