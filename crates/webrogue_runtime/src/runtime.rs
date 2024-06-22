use anyhow::Result;

pub trait IRuntime {
    fn run(&self) -> Result<()>;
}
