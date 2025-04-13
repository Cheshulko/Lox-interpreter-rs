use std::{cell::RefCell, collections::HashSet, rc::Rc};

use crate::ci::class::INIT_STR;
use crate::ci::Node;
use crate::TokenType;

use crate::ci::grammar::declaration::*;
use crate::ci::grammar::expression::*;
use crate::ci::grammar::statement::*;

struct Class<'de> {
    #[allow(dead_code)]
    name: &'de str,
    has_super_class: bool,
}

pub struct Traverser<'de> {
    var_initialization: Option<&'de str>,
    scopes_stack: Vec<HashSet<&'de str>>,
    funcs_stack: Vec<&'de str>,
    classes_stack: Vec<Class<'de>>,
}

impl<'de> Traverser<'de> {
    pub fn new() -> Self {
        Self {
            var_initialization: None,
            scopes_stack: vec![HashSet::new() /* Global scope */],
            funcs_stack: vec![],
            classes_stack: vec![],
        }
    }

    pub fn run(self, statements: &[Box<Node<'de>>]) -> anyhow::Result<()> {
        let mut result = TraverseResult::ok();

        let traverser = Rc::new(RefCell::new(self));
        for statement in statements.iter() {
            result |= statement.traverse(traverser.clone());
        }

        if result.is_own_var_initialization() {
            // TODO: Ex. [line 5] Error at 'a': Can't read local variable in its own initializer.
            anyhow::bail! {"Can't read local variable in its own initializer."}
        }
        if result.is_scope_var_redeclaretion() {
            // TODO: Ex. [line 3] Error at 'a': Already a variable with this name in this scope.
            anyhow::bail! {"Already have <var name> variable with this name in this scope"}
        }
        if result.is_invalid_return() {
            // TODO: Ex. [line 3] Error at 'return': Can't return from top-level code.
            anyhow::bail! {"Error at 'return': Can't return from top-level code."}
        }
        if result.is_this_outside_of_class() {
            // TODO: Ex. [line 1] Error at 'this': Can't use 'this' outside of a class.
            anyhow::bail! {"Error at 'this': Can't use 'this' outside of a class."}
        }
        if result.is_return_value_from_initializer() {
            // TODO: Ex. [line 5] Error at 'return': Can't return a value from an initializer.
            anyhow::bail! {"Error at 'return': Can't return a value from an initializer."}
        }
        if result.is_itself_inheritence() {
            // TODO: Ex. [line 2] Error at 'Foo': A class can't inherit from itself.
            anyhow::bail! {"A class can't inherit from itself."}
        }
        if result.is_use_super_outside_of_class() {
            anyhow::bail! {"Can't use 'super' outside of a class."}
        }
        if result.is_use_super_in_class_with_no_superclass() {
            anyhow::bail! {"Can't use 'super' in a class with no superclass."}
        }

        return anyhow::Ok(());
    }

    fn is_global_scope(&self) -> bool {
        self.scopes_stack.len() == 1
    }
}

pub trait Traverse<'env> {
    fn traverse(&self, traverser: Rc<RefCell<Traverser<'env>>>) -> TraverseResult;
}

// Declarations
impl<'de> Traverse<'de> for VarDecl<'de> {
    fn traverse(&self, traverser: Rc<RefCell<Traverser<'de>>>) -> TraverseResult {
        {
            let mut traverser_ = traverser.borrow_mut();

            let scope_already_contains = traverser_
                .scopes_stack
                .last()
                .expect("Global scope always exists")
                .contains(self.name.lexeme);

            if scope_already_contains && !traverser_.is_global_scope() {
                return TraverseResult::scope_var_redeclaretion();
            }

            assert!(
                traverser_.var_initialization.is_none(),
                "Invalid initialization in other initialization"
            );
            traverser_.var_initialization = Some(self.name.lexeme);
            traverser_
                .scopes_stack
                .last_mut()
                .expect("Global scope always exists")
                .insert(self.name.lexeme);
        }

        let mut result = TraverseResult::ok();
        if let Some(initializer) = &self.initializer {
            result |= initializer.traverse(traverser.clone());
        }

        {
            let mut traverser_ = traverser.borrow_mut();
            traverser_.var_initialization = None;
        }

        return result;
    }
}
impl<'de> Traverse<'de> for FunctionDecl<'de> {
    fn traverse(&self, traverser: Rc<RefCell<Traverser<'de>>>) -> TraverseResult {
        {
            let mut traverser_ = traverser.borrow_mut();
            traverser_.scopes_stack.push(HashSet::new());

            let func_scope = traverser_
                .scopes_stack
                .last_mut()
                .expect("There should always be a func scope");

            for parameter in self.parameters.iter() {
                if func_scope.contains(&parameter.lexeme) {
                    return TraverseResult::scope_var_redeclaretion();
                }

                func_scope.insert(parameter.lexeme);
            }

            traverser_.funcs_stack.push(self.name.lexeme);
        }

        let result = self.body.traverse(traverser.clone());

        {
            let mut traverser_ = traverser.borrow_mut();
            traverser_.scopes_stack.pop();
            traverser_.funcs_stack.pop();
        }

        // TODO: scope func redeclaretion ?

        return result;
    }
}
impl<'de> Traverse<'de> for ClassMethodDecl<'de> {
    fn traverse(&self, traverser: Rc<RefCell<Traverser<'de>>>) -> TraverseResult {
        {
            let mut traverser_ = traverser.borrow_mut();
            traverser_.scopes_stack.push(HashSet::new());

            let func_scope = traverser_
                .scopes_stack
                .last_mut()
                .expect("There should always be a func scope");

            for parameter in self.parameters.iter() {
                if func_scope.contains(&parameter.lexeme) {
                    return TraverseResult::scope_var_redeclaretion();
                }

                func_scope.insert(parameter.lexeme);
            }

            traverser_.funcs_stack.push(self.name.lexeme);
        }

        let result = self.body.traverse(traverser.clone());

        {
            let mut traverser_ = traverser.borrow_mut();
            traverser_.scopes_stack.pop();
            traverser_.funcs_stack.pop();
        }

        // TODO: scope func redeclaretion ?

        return result;
    }
}
impl<'de> Traverse<'de> for ClassDecl<'de> {
    fn traverse(&self, traverser: Rc<RefCell<Traverser<'de>>>) -> TraverseResult {
        let mut result = TraverseResult::ok();

        let has_super_class = if let Some(super_class) = self.super_class.as_ref() {
            if super_class.name.lexeme == self.name.lexeme {
                return TraverseResult::itself_inheritence();
            }

            true
        } else {
            false
        };

        {
            let mut traverser_ = traverser.borrow_mut();
            traverser_.classes_stack.push(Class {
                name: self.name.lexeme,
                has_super_class,
            });
        }

        for method in self.methods.iter() {
            result |= method.traverse(traverser.clone())
        }

        {
            let mut traverser_ = traverser.borrow_mut();
            traverser_.classes_stack.pop();
        }

        return result;
    }
}
impl<'de> Traverse<'de> for SuperClassDecl<'de> {
    fn traverse(&self, #[allow(unused)] traverser: Rc<RefCell<Traverser<'de>>>) -> TraverseResult {
        return TraverseResult::ok();
    }
}

// Expressions
impl<'de> Traverse<'de> for NilExp {
    fn traverse(&self, #[allow(unused)] traverser: Rc<RefCell<Traverser<'de>>>) -> TraverseResult {
        return TraverseResult::ok();
    }
}
impl<'de> Traverse<'de> for LiteralExp<'de> {
    fn traverse(&self, traverser: Rc<RefCell<Traverser<'de>>>) -> TraverseResult {
        let mut result = TraverseResult::ok();

        let traverser_ = traverser.borrow();
        match self.name.token_type {
            TokenType::IDENTIFIER => {
                let name = self.name.lexeme;

                if traverser_
                    .var_initialization
                    .map(|var_name| var_name == name)
                    .unwrap_or(false)
                    && !traverser_.is_global_scope()
                {
                    result |= TraverseResult::own_var_initialization();
                }
            }
            _ => {}
        }

        return result;
    }
}
impl<'de> Traverse<'de> for GroupingExp<'de> {
    fn traverse(&self, traverser: Rc<RefCell<Traverser<'de>>>) -> TraverseResult {
        return self.inner.traverse(traverser);
    }
}
impl<'de> Traverse<'de> for UnaryExp<'de> {
    fn traverse(&self, #[allow(unused)] traverser: Rc<RefCell<Traverser<'de>>>) -> TraverseResult {
        return TraverseResult::ok();
    }
}
impl<'de> Traverse<'de> for BinaryExp<'de> {
    fn traverse(&self, #[allow(unused)] traverser: Rc<RefCell<Traverser<'de>>>) -> TraverseResult {
        return TraverseResult::ok();
    }
}
impl<'de> Traverse<'de> for LogicalExp<'de> {
    fn traverse(&self, #[allow(unused)] traverser: Rc<RefCell<Traverser<'de>>>) -> TraverseResult {
        return TraverseResult::ok();
    }
}
impl<'de> Traverse<'de> for AssignmentExp<'de> {
    fn traverse(&self, traverser: Rc<RefCell<Traverser<'de>>>) -> TraverseResult {
        return self.value.traverse(traverser);
    }
}
impl<'de> Traverse<'de> for CallExp<'de> {
    fn traverse(&self, traverser: Rc<RefCell<Traverser<'de>>>) -> TraverseResult {
        let mut result = TraverseResult::ok();

        result |= self.callee.traverse(traverser.clone());

        for arg in self.args.iter() {
            result |= arg.traverse(traverser.clone());
        }

        return result;
    }
}
impl<'de> Traverse<'de> for GetExp<'de> {
    fn traverse(&self, traverser: Rc<RefCell<Traverser<'de>>>) -> TraverseResult {
        return self.callee.traverse(traverser);
    }
}
impl<'de> Traverse<'de> for SetExp<'de> {
    fn traverse(&self, traverser: Rc<RefCell<Traverser<'de>>>) -> TraverseResult {
        let mut result = TraverseResult::ok();

        result |= self.get_exp.traverse(traverser.clone());
        result |= self.value.traverse(traverser);

        return result;
    }
}
impl<'de> Traverse<'de> for ThisExp<'de> {
    fn traverse(&self, traverser: Rc<RefCell<Traverser<'de>>>) -> TraverseResult {
        let traverser_ = traverser.borrow();
        if traverser_.classes_stack.is_empty() {
            return TraverseResult::this_outside_of_class();
        } else {
            return TraverseResult::ok();
        }
    }
}
impl<'de> Traverse<'de> for SuperExp<'de> {
    fn traverse(&self, traverser: Rc<RefCell<Traverser<'de>>>) -> TraverseResult {
        let traverser_ = traverser.borrow();

        if let Some(class) = traverser_.classes_stack.last() {
            if class.has_super_class {
                return TraverseResult::ok();
            } else {
                return TraverseResult::use_super_in_class_with_no_superclass();
            }
        } else {
            return TraverseResult::use_super_outside_of_class();
        }
    }
}

// Statements
impl<'de> Traverse<'de> for EmptyStm {
    fn traverse(&self, #[allow(unused)] traverser: Rc<RefCell<Traverser<'de>>>) -> TraverseResult {
        return TraverseResult::ok();
    }
}
impl<'de> Traverse<'de> for PrintStm<'de> {
    fn traverse(&self, traverser: Rc<RefCell<Traverser<'de>>>) -> TraverseResult {
        return self.expression.traverse(traverser);
    }
}
impl<'de> Traverse<'de> for BlockStm<'de> {
    fn traverse(&self, traverser: Rc<RefCell<Traverser<'de>>>) -> TraverseResult {
        {
            let mut traverser_ = traverser.borrow_mut();
            traverser_.scopes_stack.push(HashSet::new());
        }

        let mut result = TraverseResult::ok();
        for statement in self.statements.iter() {
            result |= statement.traverse(traverser.clone());
        }

        {
            let mut traverser_ = traverser.borrow_mut();
            traverser_.scopes_stack.pop();
        }

        return result;
    }
}
impl<'de> Traverse<'de> for FuncBodyStm<'de> {
    fn traverse(&self, traverser: Rc<RefCell<Traverser<'de>>>) -> TraverseResult {
        // Note: do not create a new scope for `FuncBodyStm` because func scope has been already
        // created in `FunctionDecl`
        let mut result = TraverseResult::ok();
        for statement in self.statements.iter() {
            result |= statement.traverse(traverser.clone());
        }

        return result;
    }
}
impl<'de> Traverse<'de> for IfElseStm<'de> {
    fn traverse(&self, traverser: Rc<RefCell<Traverser<'de>>>) -> TraverseResult {
        let mut result = TraverseResult::ok();

        result |= self.then_branch.traverse(traverser.clone());
        if let Some(else_branch) = &self.else_branch {
            result |= else_branch.traverse(traverser);
        }

        return result;
    }
}
impl<'de> Traverse<'de> for WhileStm<'de> {
    fn traverse(&self, traverser: Rc<RefCell<Traverser<'de>>>) -> TraverseResult {
        return self.body.traverse(traverser);
    }
}
impl<'de> Traverse<'de> for ExpressionStm<'de> {
    fn traverse(&self, traverser: Rc<RefCell<Traverser<'de>>>) -> TraverseResult {
        return self.expression.traverse(traverser);
    }
}
impl<'de> Traverse<'de> for ReturnStm<'de> {
    fn traverse(&self, traverser: Rc<RefCell<Traverser<'de>>>) -> TraverseResult {
        {
            let traverser_ = traverser.borrow();
            if traverser_.funcs_stack.is_empty() {
                return TraverseResult::invalid_return();
            }
        }

        {
            let is_constructor = {
                let traverser_ = traverser.borrow();
                traverser_.funcs_stack.last() == Some(&INIT_STR)
            };

            match self.expression.as_ref() {
                Node::NilExp(_) => self.expression.traverse(traverser),
                _ if is_constructor => {
                    TraverseResult::return_value_from_initializer()
                        | self.expression.traverse(traverser)
                }
                _ => self.expression.traverse(traverser),
            }
        }
    }
}

pub struct TraverseResult {
    state: u8,
}

impl TraverseResult {
    fn ok() -> TraverseResult {
        Self {
            state: TraverseResult::OK,
        }
    }

    fn own_var_initialization() -> TraverseResult {
        Self {
            state: TraverseResult::OWN_VAR_INITIALIZATION,
        }
    }

    fn scope_var_redeclaretion() -> TraverseResult {
        Self {
            state: TraverseResult::SCOPE_VAR_REDECLARETION,
        }
    }

    fn invalid_return() -> TraverseResult {
        Self {
            state: TraverseResult::INVALID_RETURN,
        }
    }

    fn this_outside_of_class() -> TraverseResult {
        Self {
            state: TraverseResult::THIS_OUTSIDE_OF_CLASS,
        }
    }

    fn return_value_from_initializer() -> TraverseResult {
        Self {
            state: TraverseResult::RETURN_VALUE_FROM_INITIALIZER,
        }
    }

    fn itself_inheritence() -> TraverseResult {
        Self {
            state: TraverseResult::ITSELF_INHERITENCE,
        }
    }

    fn use_super_outside_of_class() -> TraverseResult {
        Self {
            state: TraverseResult::USE_SUPER_OUTSIDE_OF_CLASS,
        }
    }

    fn use_super_in_class_with_no_superclass() -> TraverseResult {
        Self {
            state: TraverseResult::USE_SUPER_IN_CLASS_WITH_NO_SUPERCLASS,
        }
    }

    fn is_own_var_initialization(&self) -> bool {
        self.state & TraverseResult::OWN_VAR_INITIALIZATION > 0
    }

    fn is_scope_var_redeclaretion(&self) -> bool {
        self.state & TraverseResult::SCOPE_VAR_REDECLARETION > 0
    }

    fn is_invalid_return(&self) -> bool {
        self.state & TraverseResult::INVALID_RETURN > 0
    }

    fn is_this_outside_of_class(&self) -> bool {
        self.state & TraverseResult::THIS_OUTSIDE_OF_CLASS > 0
    }

    fn is_return_value_from_initializer(&self) -> bool {
        self.state & TraverseResult::RETURN_VALUE_FROM_INITIALIZER > 0
    }

    fn is_itself_inheritence(&self) -> bool {
        self.state & TraverseResult::ITSELF_INHERITENCE > 0
    }

    fn is_use_super_outside_of_class(&self) -> bool {
        self.state & TraverseResult::USE_SUPER_OUTSIDE_OF_CLASS > 0
    }

    fn is_use_super_in_class_with_no_superclass(&self) -> bool {
        self.state & TraverseResult::USE_SUPER_IN_CLASS_WITH_NO_SUPERCLASS > 0
    }

    const OK: u8 = 0;
    const OWN_VAR_INITIALIZATION: u8 = 1 << 0;
    const SCOPE_VAR_REDECLARETION: u8 = 1 << 1;
    const INVALID_RETURN: u8 = 1 << 2;
    const THIS_OUTSIDE_OF_CLASS: u8 = 1 << 3;
    const RETURN_VALUE_FROM_INITIALIZER: u8 = 1 << 4;
    const ITSELF_INHERITENCE: u8 = 1 << 5;
    const USE_SUPER_OUTSIDE_OF_CLASS: u8 = 1 << 6;
    const USE_SUPER_IN_CLASS_WITH_NO_SUPERCLASS: u8 = 1 << 7;
}

impl std::ops::BitOrAssign for TraverseResult {
    fn bitor_assign(&mut self, rhs: Self) {
        self.state = self.state | rhs.state;
    }
}

impl std::ops::BitOr for TraverseResult {
    type Output = TraverseResult;

    fn bitor(self, rhs: Self) -> Self::Output {
        TraverseResult {
            state: self.state | rhs.state,
        }
    }
}
