use std::{cell::RefCell, rc::Rc};

use crate::{
    ci::{interpreter::Interruption, Environment, Evaluation, Interpret, Node},
    Token,
};

use super::Callable;

#[derive(Clone)]
pub struct CustomFunction<'de> {
    pub name: &'de str,
    pub arity: usize,
    pub parameters: Vec<Token<'de>>,
    // TODO: Should be weak when(if) persistent environment will be introduced
    pub captured_environment: Rc<RefCell<Environment<'de>>>,
    pub body: Rc<Node<'de>>,
}

impl<'de> CustomFunction<'de> {
    pub fn call_with_environment(
        &self,
        environment: Rc<RefCell<Environment<'de>>>,
        arguments: impl IntoIterator<Item = Evaluation<'de>>,
    ) -> Result<Evaluation<'de>, Interruption<'de>> {
        // Create a new instance to easily set arguments into the environment
        let captured_environment = Rc::new(RefCell::new(Environment::new(environment)));

        // TODO: check len(parameters) != len(arguments) ?
        self.parameters
            .iter()
            .zip(arguments)
            .for_each(|(parameter, argument)| {
                captured_environment
                    .borrow_mut()
                    .define(parameter.lexeme, argument);
            });

        match self.body.interpret(captured_environment) {
            Err(Interruption::Return(evaluation)) => Ok(evaluation),
            result => result,
        }
    }
}

impl<'de> Callable<'de> for CustomFunction<'de> {
    fn call(
        &self,
        arguments: impl IntoIterator<Item = Evaluation<'de>>,
    ) -> Result<Evaluation<'de>, Interruption<'de>> {
        self.call_with_environment(self.captured_environment.clone(), arguments)
    }
}
