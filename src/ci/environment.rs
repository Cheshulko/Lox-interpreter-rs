use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{function::native::native_functions, interpreter};

#[macro_export]
macro_rules! get_environment_value_typed {
    ($map:expr, $key:expr, {
        $( $enum_pat:pat => $val:expr ),+ $(,)?
    },
    else => $handler:expr) => {{
        match $map.get($key) {
            None => $handler,
            Some(value) => match value {
                $( $enum_pat => $val, )+
                #[allow(unreachable_patterns)]
                _ => $handler,
            }
        }
    }};
}

#[derive(Debug, Clone)]
pub struct Environment<'de> {
    values: HashMap<&'de str, interpreter::Evaluation<'de>>,
    functions: HashMap<&'de str, interpreter::Evaluation<'de>>,

    enclosing: Option<Rc<RefCell<Environment<'de>>>>,

    is_global_scope: bool,
}

impl<'a, 'de> Default for Environment<'de> {
    fn default() -> Self {
        Self {
            values: Default::default(),
            functions: Default::default(),
            enclosing: None,
            is_global_scope: false,
        }
    }
}

impl<'de> Environment<'de> {
    pub(crate) fn root() -> Self {
        Environment {
            values: Default::default(),
            functions: native_functions(),
            enclosing: None,
            is_global_scope: true,
        }
    }

    pub(crate) fn new(enclosing: Rc<RefCell<Environment<'de>>>) -> Self {
        Environment {
            values: Default::default(),
            functions: Default::default(),
            enclosing: Some(enclosing),
            is_global_scope: false,
        }
    }

    pub(crate) fn is_global_scope(&self) -> bool {
        self.is_global_scope
    }

    pub(crate) fn define(&mut self, name: &'de str, value: interpreter::Evaluation<'de>) {
        // TODO: check what if we are defining var that is already defined
        let _ = self.values.insert(name, value);
    }

    pub(crate) fn set(&mut self, name: &str, value: interpreter::Evaluation<'de>) -> bool {
        if let Some(var) = self.values.get_mut(name) {
            *var = value;

            return true;
        } else {
            if let Some(enclosing) = self.enclosing.clone() {
                if enclosing.borrow_mut().set(name, value.clone()) {
                    return true;
                }
            }
            return false;
            // TODO: Should it be an error if we set a value that is not in the environment ?
        }
    }

    pub(crate) fn get(&self, name: &str) -> Option<interpreter::Evaluation<'de>> {
        if let Some(var) = self.values.get(name) {
            return Some(var.clone());
        } else if let Some(function) = self.functions.get(name) {
            return Some(function.clone());
        } else {
            if let Some(enclosing) = self.enclosing.clone() {
                let enclosing = enclosing.borrow();

                if let Some(result) = enclosing.get(name) {
                    return Some(result);
                }
            }
            return None;
        }
    }
}

pub(crate) fn persist_environment<'de>(
    environment: Rc<RefCell<Environment<'de>>>,
) -> Rc<RefCell<Environment<'de>>> {
    // TODO: That is a shit. Should be persistent ..
    let captured_environment = {
        let environment_ = environment.borrow();

        if environment_.is_global_scope() {
            environment.clone()
        } else {
            let deep_clone_env = environment_.clone();
            Rc::new(RefCell::new(deep_clone_env))
        }
    };

    return captured_environment;
}
