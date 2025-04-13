use std::{cell::RefCell, rc::Rc};

use crate::ci::{Environment, Function};

use super::ClassInstance;

pub(crate) const INIT_STR: &'static str = "init";

#[derive(Clone)]
pub struct ClassMethod<'de> {
    pub func: Rc<Function<'de>>,
    pub class_instance: Rc<RefCell<ClassInstance<'de>>>,
    pub method_environment: Rc<RefCell<Environment<'de>>>,
}

impl<'de> ClassMethod<'de> {
    pub fn is_constructor(&self) -> bool {
        self.func.name() == INIT_STR
    }
}
