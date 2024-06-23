use anyhow::Result;

pub trait Runtime {
    fn run(&self, bytecode: Vec<u8>) -> Result<()>;
}
