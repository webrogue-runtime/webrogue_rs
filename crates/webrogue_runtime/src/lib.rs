mod backend;
mod context;
mod lifecycle;
mod memory;
mod raw_value;
mod runtime;

pub mod imported_functions;

pub use backend::Backend;
pub use context::{Context, MemoryFactory};
pub use lifecycle::Lifecycle;
pub use runtime::Runtime;
pub use wiggle::{DynamicGuestMemory, GuestMemory};
