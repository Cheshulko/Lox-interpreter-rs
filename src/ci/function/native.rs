use std::{
    collections::HashMap,
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    ci::{interpreter::Interruption, Evaluation},
    Token,
};

use super::{Callable, Function};

pub struct NativeFunction<'de> {
    pub name: &'static str,
    pub arity: usize,
    #[allow(unused)]
    pub parameters: Vec<Token<'de>>,
    pub body: Rc<dyn Fn() -> Result<Evaluation<'de>, Interruption<'de>>>,
}

impl<'de> Callable<'de> for NativeFunction<'de> {
    fn call(
        &self,
        _arguments: impl IntoIterator<Item = Evaluation<'de>>,
    ) -> Result<Evaluation<'de>, Interruption<'de>> {
        (self.body)()
    }
}

// TODO: static
pub fn native_functions<'de>() -> HashMap<&'de str, Evaluation<'de>> {
    let clock_fn = Rc::new(Function::NativeFunction(Box::new(NativeFunction {
        name: "clock",
        arity: 0,
        parameters: vec![],
        body: Rc::new(|| {
            let now = SystemTime::now();
            let result = match now.duration_since(UNIX_EPOCH) {
                Ok(duration) => duration.as_secs(),
                Err(e) => {
                    return Err(Interruption::Error(
                        anyhow::anyhow! {"Could not get clock: {e}"},
                    ))
                }
            };

            return Ok(Evaluation::Number(result as f64));
        }),
    })));

    [("clock", Evaluation::Fn(clock_fn))].into_iter().collect()
}
