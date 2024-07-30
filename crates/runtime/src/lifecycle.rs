use crate::backend::Backend;
pub use anyhow::Result;

pub struct Lifecycle {}

impl Lifecycle {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run<Imports>(
        &self,
        backend: impl Backend<Imports>,
        imports: Imports,
        context_vec: crate::context::ContextVec,
        archive_reader: webrogue_wrapp::Reader,
    ) -> Result<()> {
        let mut archive_reader = archive_reader;
        let runtime = backend.make_runtime();
        runtime.run(imports, context_vec, archive_reader.read_wasm()?)?;
        Ok(())
    }
}
