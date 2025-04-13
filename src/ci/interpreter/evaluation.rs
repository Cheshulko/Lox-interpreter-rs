use std::cell::RefCell;
use std::rc::Rc;

use crate::ci::class::ClassMethod;
use crate::ci::Function;
use crate::ci::{Class, ClassInstance};

#[derive(Clone)]
pub enum Evaluation<'de> {
    Number(f64),
    Boolean(bool),
    // TODO: Staric storage for string literals
    // TODO: Remove owning
    String(String),
    Nil,
    Fn(Rc<Function<'de>>),
    Class(Rc<Class<'de>>),
    ClassMethod(ClassMethod<'de>),
    ClassInstance(Rc<RefCell<ClassInstance<'de>>>),
    None,
}

impl std::fmt::Debug for Evaluation<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(arg0) => f.debug_tuple("Number").field(arg0).finish(),
            Self::Boolean(arg0) => f.debug_tuple("Boolean").field(arg0).finish(),
            Self::String(arg0) => f.debug_tuple("String").field(arg0).finish(),
            Self::Nil => write!(f, "Nil"),
            Self::Fn(arg0) => write!(f, "Fn: {}", arg0.name(),),
            Self::Class(arg0) => write!(f, "Class: {}", arg0.name),
            Self::ClassMethod(arg0) => write!(f, "Method: {}", arg0.func.name()),
            Self::ClassInstance(arg0) => write!(f, "{} instance", arg0.borrow().class().name),
            Self::None => write!(f, "None"),
        }
    }
}

impl std::fmt::Display for Evaluation<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Evaluation::Number(n) => write!(f, "{n}"),
            Evaluation::Boolean(true) => write!(f, "true"),
            Evaluation::Boolean(false) => write!(f, "false"),
            Evaluation::Nil => write!(f, "nil"),
            Evaluation::String(s) => write!(f, "{s}"),
            Evaluation::Fn(s) => write!(f, "<fn {}>", s.name()),
            Evaluation::Class(c) => write!(f, "{}", c.name,),
            Evaluation::ClassMethod(m) => write!(f, "<method {}>", m.func.name()),
            Evaluation::ClassInstance(ci) => write!(f, "{} instance", ci.borrow().class().name),
            Evaluation::None => Ok(()),
        }
    }
}
