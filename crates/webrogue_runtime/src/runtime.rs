use anyhow::Result;

pub trait Runtime {
    fn run(&self) -> Result<()>;
}
