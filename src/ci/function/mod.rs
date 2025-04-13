pub(crate) mod custom;
pub(crate) mod native;

use custom::CustomFunction;
use native::NativeFunction;

use super::{interpreter::Interruption, Evaluation};

pub enum Function<'de> {
    CustomFunction(Box<CustomFunction<'de>>),
    NativeFunction(Box<NativeFunction<'de>>),
}

impl<'de> Function<'de> {
    pub fn name(&self) -> &'de str {
        match self {
            Function::CustomFunction(custom_function) => custom_function.name,
            Function::NativeFunction(native_function) => native_function.name,
        }
    }

    pub fn arity(&self) -> usize {
        match self {
            Function::CustomFunction(callable) => callable.arity,
            Function::NativeFunction(callable) => callable.arity,
        }
    }
}

pub trait Callable<'de> {
    fn call(
        &self,
        arguments: impl IntoIterator<Item = Evaluation<'de>>,
    ) -> Result<Evaluation<'de>, Interruption<'de>>;
}

impl<'de> Callable<'de> for Function<'de> {
    fn call(
        &self,
        arguments: impl IntoIterator<Item = Evaluation<'de>>,
    ) -> Result<Evaluation<'de>, Interruption<'de>> {
        match self {
            Function::CustomFunction(custom_function) => custom_function.call(arguments),
            Function::NativeFunction(native_function) => native_function.call(arguments),
        }
    }
}
