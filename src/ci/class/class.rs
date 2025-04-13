use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::ci::Environment;

pub(crate) const CLASS_STR: &'static str = "class";

#[derive(Clone)]
pub struct Class<'de> {
    pub name: &'de str,
    pub super_class: Option<Weak<Class<'de>>>,
    // TODO: Should be weak when(if) persistent environment will be introduced
    pub class_environment: Rc<RefCell<Environment<'de>>>,
}
