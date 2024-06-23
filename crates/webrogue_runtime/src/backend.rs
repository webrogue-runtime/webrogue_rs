use crate::runtime::Runtime;

pub trait Backend {
    fn make_runtime(&self) -> Box<dyn Runtime>;
}
