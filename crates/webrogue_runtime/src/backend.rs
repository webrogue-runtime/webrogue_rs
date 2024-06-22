use crate::runtime::IRuntime;

pub trait IBackend {
    fn make_runtime(&self) -> Box<dyn IRuntime>;
}
