use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

use crate::{ci::Evaluation, get_environment_value_typed};

use super::{method::INIT_STR, Class, ClassMethod};

pub(crate) const THIS_STR: &'static str = "this";

pub struct ClassInstance<'de> {
    class: Rc<Class<'de>>,
    fields: HashMap<&'de str, Evaluation<'de>>,

    weak_self: Weak<RefCell<ClassInstance<'de>>>,
}

impl<'de> ClassInstance<'de> {
    pub(crate) fn new_rc(class: Rc<Class<'de>>) -> Rc<RefCell<ClassInstance<'de>>> {
        let class_instance = Rc::new(RefCell::new(Self {
            class,
            fields: HashMap::new(),
            weak_self: Weak::new(),
        }));

        {
            let mut class_instance_ = class_instance.borrow_mut();
            class_instance_.weak_self = Rc::downgrade(&class_instance);
        }

        class_instance
    }

    pub fn class(&self) -> &Class<'de> {
        &self.class
    }

    pub fn get_field(&self, name: &'de str) -> Result<Evaluation<'de>, anyhow::Error> {
        self.get_filed_for_class(self.class.clone(), name)
    }

    pub fn get_filed_for_class(
        &self,
        class: Rc<Class<'de>>,
        name: &'de str,
    ) -> Result<Evaluation<'de>, anyhow::Error> {
        let class_environment = class.class_environment.clone();
        let class_environment_ = class_environment.borrow();

        // Ok. That is a field (aka a property)
        if let Some(evaluation) = self.fields.get(name) {
            return Ok(evaluation.clone());
        }

        // As we do not have static methods, there should always be a `class_instance` for the method call
        let class_instance = self
            .weak_self
            .upgrade()
            .ok_or(anyhow::anyhow! {"No `self class instance` for class method call"})?;

        return get_environment_value_typed!(class_environment_, name, {
            Evaluation::Fn(func) => {
                Ok(Evaluation::ClassMethod(ClassMethod {
                    func,
                    class_instance,
                    method_environment: class_environment.clone(),
                }))
            }
        }, else => {
            // Searching the method at super classes
            self.get_field_super(name)
        });
    }

    fn get_field_super(&self, name: &'de str) -> Result<Evaluation<'de>, anyhow::Error> {
        let mut super_class = self.class.super_class.clone();

        // As we do not have static methods, there should always be a `class_instance` for the method call
        let class_instance = self
            .weak_self
            .upgrade()
            .ok_or(anyhow::anyhow! {"No `self class instance` for class method call"})?;

        while let Some(super_class_) = super_class {
            let super_class_ = super_class_
                .upgrade()
                .expect("Should always be a valid super class");
            let super_class_environment = super_class_.class_environment.clone();
            let super_class_environment_ = super_class_environment.borrow();

            get_environment_value_typed!(super_class_environment_, name, {
                Evaluation::Fn(func) => {
                    return Ok(Evaluation::ClassMethod(ClassMethod {
                        func,
                        class_instance,
                        method_environment: super_class_environment.clone(),
                    }))
                }
            }, else => {
                // Trying to search deeper at super class hierarchy
                super_class = super_class_.super_class.clone();
            });
        }

        anyhow::bail! {"Undefined property '{}'.", name}
    }

    pub fn set_field(&mut self, name: &'de str, value: Evaluation<'de>) {
        let _ = self.fields.insert(name, value);
    }

    pub fn get_constructor(&self) -> Result<Evaluation<'de>, anyhow::Error> {
        self.get_field(INIT_STR)
    }
}
