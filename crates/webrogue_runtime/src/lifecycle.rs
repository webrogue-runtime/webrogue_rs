use crate::backend::IBackend;
pub use anyhow::{Error, Result};

pub struct Lifecycle {}

impl Lifecycle {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self, backend: impl IBackend) -> Result<()> {
        let runtime = backend.make_runtime();
        runtime.run()?;
        Ok(())
    }
}
