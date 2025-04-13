use std::{cell::RefCell, process::exit, rc::Rc};

use crate::{
    ci::{
        class::{ClassMethod, CLASS_STR, THIS_STR},
        environment::persist_environment,
        function::{custom::CustomFunction, Callable},
        Class, ClassInstance, Environment, Function, Node,
    },
    get_environment_value_typed, TokenType,
};

use crate::ci::grammar::declaration::*;
use crate::ci::grammar::expression::*;
use crate::ci::grammar::statement::*;

use super::{Evaluation, Interruption};

pub struct Interpreter<'de> {
    global_environment: Rc<RefCell<Environment<'de>>>,
    statements: Vec<Box<Node<'de>>>,
}

impl<'de> Interpreter<'de> {
    pub fn new(statements: impl IntoIterator<Item = Box<Node<'de>>>) -> Self {
        Self {
            statements: statements.into_iter().collect(),
            global_environment: Rc::new(RefCell::new(Environment::root())),
        }
    }
    pub fn run(&mut self) {
        for statement in self.statements.iter() {
            match statement.interpret(self.global_environment.clone()) {
                Ok(_) => {}
                Err(error) => {
                    eprintln!("{error}");
                    exit(70);
                }
            }
        }
    }
}

pub trait Interpret<'env> {
    fn interpret(
        &self,
        environment: Rc<RefCell<Environment<'env>>>,
    ) -> Result<Evaluation<'env>, Interruption<'env>>;
}

// Declarations
impl<'de> Interpret<'de> for VarDecl<'de> {
    fn interpret(
        &self,
        environment: Rc<RefCell<Environment<'de>>>,
    ) -> Result<Evaluation<'de>, Interruption<'de>> {
        if let Some(initializer) = &self.initializer {
            let initialization = initializer.interpret(environment.clone())?;

            let mut environment_ = environment.borrow_mut();
            environment_.define(self.name.lexeme, initialization);
        } else {
            let mut environment_ = environment.borrow_mut();
            environment_.define(self.name.lexeme, Evaluation::Nil);
        }

        return Ok(Evaluation::None);
    }
}
impl<'de> Interpret<'de> for FunctionDecl<'de> {
    fn interpret(
        &self,
        environment: Rc<RefCell<Environment<'de>>>,
    ) -> Result<Evaluation<'de>, Interruption<'de>> {
        let captured_environment = persist_environment(environment.clone());

        let body = self.body.clone();
        let func = Rc::new(Function::CustomFunction(Box::new(CustomFunction {
            parameters: self.parameters.clone(),
            name: &self.name.lexeme,
            arity: self.parameters.len(),
            captured_environment,
            body,
        })));

        {
            let mut environment_ = environment.borrow_mut();

            environment_.define(self.name.lexeme, Evaluation::Fn(func));
        }

        return Ok(Evaluation::Nil);
    }
}
impl<'de> Interpret<'de> for ClassMethodDecl<'de> {
    fn interpret(
        &self,
        environment: Rc<RefCell<Environment<'de>>>,
    ) -> Result<Evaluation<'de>, Interruption<'de>> {
        let body = self.body.clone();
        let func = Rc::new(Function::CustomFunction(Box::new(CustomFunction {
            parameters: self.parameters.clone(),
            name: &self.name.lexeme,
            arity: self.parameters.len(),
            captured_environment: environment.clone(),
            body,
        })));

        {
            let mut environment_ = environment.borrow_mut();

            get_environment_value_typed!(environment_, CLASS_STR, {
                Evaluation::Class(_) => {
                    environment_.define(self.name.lexeme, Evaluation::Fn(func))
                }
            }, else => {
                unreachable!(
                    "Should not be a case because `class` is always `Class` evaluation",
                )
            });
        }

        return Ok(Evaluation::Nil);
    }
}
impl<'de> Interpret<'de> for ClassDecl<'de> {
    fn interpret(
        &self,
        environment: Rc<RefCell<Environment<'de>>>,
    ) -> Result<Evaluation<'de>, Interruption<'de>> {
        let captured_environment = Rc::new(RefCell::new(Environment::new(environment.clone())));
        let captured_environment = persist_environment(captured_environment.clone());

        let super_class = {
            if let Some(super_class) = self.super_class.as_ref() {
                let super_class = super_class.interpret(environment.clone())?;
                match super_class {
                    Evaluation::Class(class) => Some(Rc::downgrade(&class)),
                    _ => {
                        return Err(Interruption::Error(
                            anyhow::anyhow! {"Super class {} should be a class", self.name.lexeme},
                        ))
                    }
                }
            } else {
                None
            }
        };

        let class_evaluation = Evaluation::Class(Rc::new(Class {
            name: self.name.lexeme,
            super_class,
            class_environment: captured_environment.clone(),
        }));

        {
            let mut captured_environment_ = captured_environment.borrow_mut();
            captured_environment_.define(CLASS_STR, class_evaluation.clone());
        }

        {
            let mut environment_ = environment.borrow_mut();
            environment_.define(self.name.lexeme, class_evaluation);
        }

        for method in self.methods.iter() {
            match method.as_ref() {
                Node::ClassMethodDecl(_) => {
                    let _ = method.interpret(captured_environment.clone())?;
                }
                _ => {
                    return Err(Interruption::Error(
                        anyhow::anyhow! {"Only class method declarations are valid for class methods"},
                    ))
                }
            };
        }

        return Ok(Evaluation::Nil);
    }
}
impl<'de> Interpret<'de> for SuperClassDecl<'de> {
    fn interpret(
        &self,
        environment: Rc<RefCell<Environment<'de>>>,
    ) -> Result<Evaluation<'de>, Interruption<'de>> {
        let super_class = {
            let environment_ = environment.borrow();

            environment_
                .get(self.name.lexeme)
                .ok_or(anyhow::anyhow! {"Not defined a super class {}", self.name.lexeme})?
        };

        match super_class {
            Evaluation::Class(class) => {
                return Ok(Evaluation::Class(class));
            }
            _ => {
                return Err(Interruption::Error(
                    anyhow::anyhow! {"Super class {} should be a class", self.name.lexeme},
                ))
            }
        }
    }
}

// Expressions
impl<'de> Interpret<'de> for NilExp {
    fn interpret(
        &self,
        #[allow(unused)] environment: Rc<RefCell<Environment<'de>>>,
    ) -> Result<Evaluation<'de>, Interruption<'de>> {
        return Ok(Evaluation::Nil);
    }
}
impl<'de> Interpret<'de> for LiteralExp<'de> {
    fn interpret(
        &self,
        environment: Rc<RefCell<Environment<'de>>>,
    ) -> Result<Evaluation<'de>, Interruption<'de>> {
        return match self.name.token_type {
            TokenType::NUMBER(n) => Ok(Evaluation::Number(n)),
            TokenType::TRUE => Ok(Evaluation::Boolean(true)),
            TokenType::FALSE => Ok(Evaluation::Boolean(false)),
            TokenType::NIL => Ok(Evaluation::Nil),
            TokenType::STRING(ref s) => Ok(Evaluation::String(s.clone())),
            TokenType::IDENTIFIER => {
                let name = &self.name.lexeme;
                let environment_ = environment.borrow();

                return get_environment_value_typed!(environment_, name, {
                    any => Ok(any)
                }, else => {
                    Err(Interruption::Error(
                        anyhow::anyhow! {"Not initialized IDENTIFIER: {name}", name = &self.name},
                    ))
                });
            }
            _ => {
                return Err(Interruption::Error(
                    anyhow::anyhow! {"Unsupported literal evaluation for token: {name}", name = &self.name},
                ));
            }
        };
    }
}
impl<'de> Interpret<'de> for GroupingExp<'de> {
    fn interpret(
        &self,
        environment: Rc<RefCell<Environment<'de>>>,
    ) -> Result<Evaluation<'de>, Interruption<'de>> {
        return self.inner.interpret(environment);
    }
}
impl<'de> Interpret<'de> for UnaryExp<'de> {
    fn interpret(
        &self,
        environment: Rc<RefCell<Environment<'de>>>,
    ) -> Result<Evaluation<'de>, Interruption<'de>> {
        return match self.operator.token_type {
            TokenType::BANG => match self.right.interpret(environment)? {
                Evaluation::Boolean(b) => Ok(Evaluation::Boolean(!b)),
                Evaluation::Nil => Ok(Evaluation::Boolean(true)),
                Evaluation::Number(_) => Ok(Evaluation::Boolean(false)),
                e => {
                    return Err(Interruption::Error(
                        anyhow::anyhow! {"Unsupported unary BANG for evaluation: {e}"},
                    ));
                }
            },
            TokenType::MINUS => match self.right.interpret(environment)? {
                Evaluation::Number(n) => Ok(Evaluation::Number(-1.0 * n)),
                e => {
                    return Err(Interruption::Error(
                        anyhow::anyhow! {"Unsupported unary MINUS for evaluation: {e}"},
                    ))
                }
            },
            _ => {
                return Err(Interruption::Error(
                    anyhow::anyhow! {"Unsupported unary operator: {o}", o = &self.operator},
                ))
            }
        };
    }
}
impl<'de> Interpret<'de> for BinaryExp<'de> {
    fn interpret(
        &self,
        environment: Rc<RefCell<Environment<'de>>>,
    ) -> Result<Evaluation<'de>, Interruption<'de>> {
        let left = self.left.interpret(environment.clone())?;
        let right = self.right.interpret(environment.clone())?;

        match self.operator.token_type {
            TokenType::PLUS => match (left, right) {
                (Evaluation::Number(left), Evaluation::Number(right)) => {
                    Ok(Evaluation::Number(left + right))
                }
                (Evaluation::String(left), Evaluation::String(right)) => {
                    Ok(Evaluation::String(format!("{left}{right}")))
                }
                e => {
                    return Err(Interruption::Error(
                        anyhow::anyhow! {"Unsupported binary PLUS for evaluation: {e:?}"},
                    ))
                }
            },
            TokenType::MINUS => match (left, right) {
                (Evaluation::Number(left), Evaluation::Number(right)) => {
                    Ok(Evaluation::Number(left - right))
                }
                e @ _ => {
                    return Err(Interruption::Error(
                        anyhow::anyhow! {"Unsupported binary MINUS for evaluation: {e:?}"},
                    ))
                }
            },
            TokenType::STAR => match (left, right) {
                (Evaluation::Number(left), Evaluation::Number(right)) => {
                    Ok(Evaluation::Number(left * right))
                }
                e @ _ => {
                    return Err(Interruption::Error(
                        anyhow::anyhow! {"Unsupported binary STAR for evaluation: {e:?}"},
                    ))
                }
            },
            TokenType::SLASH => match (left, right) {
                (Evaluation::Number(left), Evaluation::Number(right)) => {
                    Ok(Evaluation::Number(left / right))
                }
                e @ _ => {
                    return Err(Interruption::Error(
                        anyhow::anyhow! {"Unsupported binary SLASH for evaluation: {e:?}"},
                    ))
                }
            },
            TokenType::LESS => match (left, right) {
                (Evaluation::Number(left), Evaluation::Number(right)) => {
                    Ok(Evaluation::Boolean(left < right))
                }
                e @ _ => {
                    return Err(Interruption::Error(
                        anyhow::anyhow! {"Unsupported binary LESS for evaluation: {e:?}"},
                    ))
                }
            },
            TokenType::LESS_EQUAL => match (left, right) {
                (Evaluation::Number(left), Evaluation::Number(right)) => {
                    Ok(Evaluation::Boolean(left <= right))
                }
                e @ _ => {
                    return Err(Interruption::Error(
                        anyhow::anyhow! {"Unsupported binary LESS_EQUAL for evaluation: {e:?}"},
                    ))
                }
            },
            TokenType::GREATER => match (left, right) {
                (Evaluation::Number(left), Evaluation::Number(right)) => {
                    Ok(Evaluation::Boolean(left > right))
                }
                e @ _ => {
                    return Err(Interruption::Error(
                        anyhow::anyhow! {"Unsupported binary GREATER for evaluation: {e:?}"},
                    ))
                }
            },
            TokenType::GREATER_EQUAL => match (left, right) {
                (Evaluation::Number(left), Evaluation::Number(right)) => {
                    Ok(Evaluation::Boolean(left >= right))
                }
                e @ _ => {
                    return Err(Interruption::Error(
                        anyhow::anyhow! {"Unsupported binary GREATER_EQUAL for evaluation: {e:?}"},
                    ));
                }
            },
            TokenType::EQUAL_EQUAL => match (left, right) {
                (Evaluation::Number(left), Evaluation::Number(right)) => {
                    Ok(Evaluation::Boolean(left == right))
                }
                (Evaluation::String(left), Evaluation::String(right)) => {
                    Ok(Evaluation::Boolean(left == right))
                }
                (Evaluation::Boolean(left), Evaluation::Boolean(right)) => {
                    Ok(Evaluation::Boolean(left == right))
                }
                (Evaluation::Nil, Evaluation::Nil) => Ok(Evaluation::Boolean(true)),
                _ => Ok(Evaluation::Boolean(false)),
            },
            TokenType::BANG_EQUAL => match (left, right) {
                (Evaluation::Number(left), Evaluation::Number(right)) => {
                    Ok(Evaluation::Boolean(left != right))
                }
                (Evaluation::String(left), Evaluation::String(right)) => {
                    Ok(Evaluation::Boolean(left != right))
                }
                (Evaluation::Boolean(left), Evaluation::Boolean(right)) => {
                    Ok(Evaluation::Boolean(left != right))
                }
                (Evaluation::Nil, Evaluation::Nil) => Ok(Evaluation::Boolean(false)),
                _ => Ok(Evaluation::Boolean(true)),
            },
            _ => {
                return Err(Interruption::Error(
                    anyhow::anyhow! {"Unsupported binary operator: {o}", o = &self.operator},
                ))
            }
        }
    }
}
impl<'de> Interpret<'de> for LogicalExp<'de> {
    fn interpret(
        &self,
        environment: Rc<RefCell<Environment<'de>>>,
    ) -> Result<Evaluation<'de>, Interruption<'de>> {
        let left = self.left.interpret(environment.clone())?;

        match self.operator.token_type {
            TokenType::OR => match left {
                Evaluation::Boolean(false) | Evaluation::Nil => {
                    self.right.interpret(environment.clone())
                }
                _ => Ok(left),
            },
            TokenType::AND => match left {
                Evaluation::Boolean(false) | Evaluation::Nil => Ok(left),
                _ => self.right.interpret(environment.clone()),
            },
            _ => {
                return Err(Interruption::Error(
                    anyhow::anyhow! {"Unsupported logical operator: {o}", o = &self.operator},
                ))
            }
        }
    }
}
impl<'de> Interpret<'de> for AssignmentExp<'de> {
    fn interpret(
        &self,
        environment: Rc<RefCell<Environment<'de>>>,
    ) -> Result<Evaluation<'de>, Interruption<'de>> {
        let value = self.value.interpret(environment.clone())?;

        let mut environment_ = environment.borrow_mut();
        environment_.set(self.name.lexeme, value.clone());

        return Ok(value);
    }
}
impl<'de> Interpret<'de> for CallExp<'de> {
    fn interpret(
        &self,
        environment: Rc<RefCell<Environment<'de>>>,
    ) -> Result<Evaluation<'de>, Interruption<'de>> {
        let callee = self.callee.interpret(environment.clone())?;

        let mut args = vec![];
        for arg in self.args.iter() {
            args.push(arg.interpret(environment.clone())?);
        }

        fn call_class_method<'de>(
            class_method: ClassMethod<'de>,
            args: Vec<Evaluation<'de>>,
        ) -> Result<Evaluation<'de>, Interruption<'de>> {
            let is_constructor = class_method.is_constructor();

            let class_instance = class_method.class_instance;

            let method_environment = Rc::new(RefCell::new(Environment::new(
                class_method.method_environment.clone(),
            )));

            // Defining `this` for the method
            {
                let mut method_environment_ = method_environment.borrow_mut();
                method_environment_
                    .define(THIS_STR, Evaluation::ClassInstance(class_instance.clone()));
            }

            let result = match class_method.func.as_ref() {
                Function::CustomFunction(custom_function) => {
                    custom_function.call_with_environment(method_environment, args)
                }
                _ => unreachable!(),
            };

            if is_constructor {
                match result? {
                    Evaluation::Nil => {
                        return Ok(Evaluation::ClassInstance(class_instance));
                    }
                    _ => panic!(
                        "Returning a non-null value from a constructor should be rejected at a traversal step"
                    ),
                }
            } else {
                return result;
            }
        }

        match callee {
            Evaluation::Fn(callable) => {
                if callable.arity() != args.len() {
                    return Err(Interruption::Error(
                        anyhow::anyhow! {"Error. Expected {} arguments but have {}", callable.arity(),args.len() },
                    ));
                }
                match callable.as_ref() {
                    Function::CustomFunction(function) => function.call(args),
                    Function::NativeFunction(function) => function.call(args),
                }
            }
            Evaluation::ClassMethod(class_method) => call_class_method(class_method, args),
            Evaluation::Class(class) => {
                let class_instance = ClassInstance::new_rc(class);

                {
                    let init = {
                        let class_instance_ = class_instance.borrow();
                        class_instance_.get_constructor()
                    };

                    if let Ok(Evaluation::ClassMethod(init)) = init {
                        call_class_method(init, args)?;
                    }
                }

                Ok(Evaluation::ClassInstance(class_instance))
            }
            _ => {
                return Err(Interruption::Error(
                    anyhow::anyhow! {"{:?} is not callable", callee},
                ))
            }
        }
    }
}
impl<'de> Interpret<'de> for GetExp<'de> {
    fn interpret(
        &self,
        environment: Rc<RefCell<Environment<'de>>>,
    ) -> Result<Evaluation<'de>, Interruption<'de>> {
        let callee = self.callee.interpret(environment.clone())?;
        let name = &self.name;

        return match callee {
            Evaluation::ClassInstance(class_instance) => {
                let class_instance_ = class_instance.borrow();

                class_instance_
                    .get_field(name.lexeme)
                    .map_err(|error| Interruption::Error(error))
            }
            x => Err(Interruption::Error(
                anyhow::anyhow! {"Only instances have properties. {x:?}"},
            )),
        };
    }
}
impl<'de> Interpret<'de> for SetExp<'de> {
    fn interpret(
        &self,
        environment: Rc<RefCell<Environment<'de>>>,
    ) -> Result<Evaluation<'de>, Interruption<'de>> {
        let get_exp = match self.get_exp.as_ref() {
            Node::GetExp(get_exp) => get_exp,
            _ => return Err(Interruption::Error(anyhow::anyhow! {"TODO: WTF"})),
        };
        let calle = get_exp.callee.interpret(environment.clone())?;
        let name = &get_exp.name;
        let value = self.value.interpret(environment)?;

        return match calle {
            Evaluation::ClassInstance(class_instance) => {
                let mut class_instance_ = class_instance.borrow_mut();
                class_instance_.set_field(name.lexeme, value.clone());

                Ok(value)
            }

            _ => Err(Interruption::Error(anyhow::anyhow! {"TODO: WTF"})),
        };
    }
}
impl<'de> Interpret<'de> for ThisExp<'de> {
    fn interpret(
        &self,
        environment: Rc<RefCell<Environment<'de>>>,
    ) -> Result<Evaluation<'de>, Interruption<'de>> {
        let environment_ = environment.borrow();

        return get_environment_value_typed!(environment_, THIS_STR, {
            this => Ok(this)
        }, else => {
            Err(Interruption::Error(
                anyhow::anyhow! {"Error at 'this': Can't use 'this' outside of a class."},
            ))
        });
    }
}
impl<'de> Interpret<'de> for SuperExp<'de> {
    fn interpret(
        &self,
        environment: Rc<RefCell<Environment<'de>>>,
    ) -> Result<Evaluation<'de>, Interruption<'de>> {
        let environment_ = environment.borrow();

        let class = get_environment_value_typed!(environment_, CLASS_STR, {
                Evaluation::Class(class) => class
            },
            else => {
                return Err(Interruption::Error(
                    anyhow::anyhow! {"Error at 'super': Can't use 'super' outside of a class"},
                ));
        });

        let Some(super_class) = class
            .super_class
            .as_ref()
            .and_then(|super_class| super_class.upgrade())
        else {
            return Err(Interruption::Error(
                anyhow::anyhow! {"Class `{}` doesn't have a super class", class.name},
            ));
        };

        let this_class_instance = get_environment_value_typed!(environment_, THIS_STR, {
            Evaluation::ClassInstance(this_class_instance) => this_class_instance
        }, else => {
            return Err(Interruption::Error(
                anyhow::anyhow! {"Error at 'super': Can't use 'super' outside of a class or the class doesn't have a super class"},
            ));
        });

        {
            let this_class_instance_ = this_class_instance.borrow();

            this_class_instance_
                .get_filed_for_class(super_class, self.method.lexeme)
                .map_err(|error| Interruption::Error(error))
        }
    }
}

// Statements
impl<'de> Interpret<'de> for EmptyStm {
    fn interpret(
        &self,
        #[allow(unused)] environment: Rc<RefCell<Environment<'de>>>,
    ) -> Result<Evaluation<'de>, Interruption<'de>> {
        return Ok(Evaluation::None);
    }
}
impl<'de> Interpret<'de> for PrintStm<'de> {
    fn interpret(
        &self,
        environment: Rc<RefCell<Environment<'de>>>,
    ) -> Result<Evaluation<'de>, Interruption<'de>> {
        let result = self.expression.interpret(environment)?;
        println!("{result}");

        return Ok(Evaluation::None);
    }
}
impl<'de> Interpret<'de> for BlockStm<'de> {
    fn interpret(
        &self,
        environment: Rc<RefCell<Environment<'de>>>,
    ) -> Result<Evaluation<'de>, Interruption<'de>> {
        let block_environment = Rc::new(RefCell::new(Environment::new(environment.clone())));

        for statement in self.statements.iter() {
            let _ = statement.interpret(block_environment.clone())?;
        }

        return Ok(Evaluation::Nil);
    }
}
impl<'de> Interpret<'de> for FuncBodyStm<'de> {
    fn interpret(
        &self,
        environment: Rc<RefCell<Environment<'de>>>,
    ) -> Result<Evaluation<'de>, Interruption<'de>> {
        let block_environment = Rc::new(RefCell::new(Environment::new(environment.clone())));

        for statement in self.statements.iter() {
            let _ = statement.interpret(block_environment.clone())?;
        }

        return Ok(Evaluation::Nil);
    }
}
impl<'de> Interpret<'de> for IfElseStm<'de> {
    fn interpret(
        &self,
        environment: Rc<RefCell<Environment<'de>>>,
    ) -> Result<Evaluation<'de>, Interruption<'de>> {
        let condition = self.condition.interpret(environment.clone())?;

        return match condition {
            Evaluation::Boolean(false) | Evaluation::Nil => {
                if let Some(else_branch) = self.else_branch.as_ref() {
                    else_branch.interpret(environment)
                } else {
                    Ok(Evaluation::None)
                }
            }
            // Consider everything else as `true`
            _ => self.then_branch.interpret(environment),
        };
    }
}
impl<'de> Interpret<'de> for WhileStm<'de> {
    fn interpret(
        &self,
        environment: Rc<RefCell<Environment<'de>>>,
    ) -> Result<Evaluation<'de>, Interruption<'de>> {
        loop {
            let condition = self.condition.interpret(environment.clone())?;
            // TODO: matches!
            if !match condition {
                Evaluation::Boolean(false) | Evaluation::Nil => false,
                // Consider everything else as `true`
                _ => true,
            } {
                break;
            }

            self.body.interpret(environment.clone())?;
        }

        return Ok(Evaluation::None);
    }
}
impl<'de> Interpret<'de> for ExpressionStm<'de> {
    fn interpret(
        &self,
        environment: Rc<RefCell<Environment<'de>>>,
    ) -> Result<Evaluation<'de>, Interruption<'de>> {
        let _ = self.expression.interpret(environment)?;

        return Ok(Evaluation::None);
    }
}
impl<'de> Interpret<'de> for ReturnStm<'de> {
    fn interpret(
        &self,
        environment: Rc<RefCell<Environment<'de>>>,
    ) -> Result<Evaluation<'de>, Interruption<'de>> {
        let ev = self.expression.interpret(environment)?;

        return Err(Interruption::Return(ev));
    }
}
