use crate::backend::Backend;
pub use anyhow::{Error, Result};

pub struct Lifecycle {}

impl Lifecycle {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self, backend: impl Backend) -> Result<()> {
        let runtime = backend.make_runtime();
        runtime.run()?;
        Ok(())
    }
}
