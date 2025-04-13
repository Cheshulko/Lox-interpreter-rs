pub(crate) mod class;
pub(crate) mod instance;
pub(crate) mod method;

pub use class::Class;
pub use instance::ClassInstance;
pub use method::ClassMethod;

pub(crate) use {class::CLASS_STR, instance::THIS_STR, method::INIT_STR};
