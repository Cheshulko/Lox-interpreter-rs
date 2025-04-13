pub(crate) mod evaluation;
pub(crate) mod interpreter;
pub(crate) mod interruption;

use std::{cell::RefCell, rc::Rc};

pub use evaluation::Evaluation;
pub use interpreter::{Interpret, Interpreter};
pub use interruption::Interruption;

use super::{Environment, Node};

impl<'de> Interpret<'de> for Node<'de> {
    #[rustfmt::skip]
    fn interpret(
        &self,
        environment: Rc<RefCell<Environment<'de>>>,
    ) -> Result<Evaluation<'de>, Interruption<'de>> {
        match self {
            // Declarations
            Node::VarDecl(var_decl) => var_decl.interpret(environment),
            Node::FunctionDecl(function_decl) => function_decl.interpret(environment),
            Node::ClassMethodDecl(class_method_decl) => class_method_decl.interpret(environment),
            Node::ClassDecl(class_decl) => class_decl.interpret(environment),
            Node::SuperClassDecl(super_class_decl) => super_class_decl.interpret(environment),

            // Expressions
            Node::NilExp(nil_exp) => nil_exp.interpret(environment),
            Node::LiteralExp(literal_exp) => literal_exp.interpret(environment),
            Node::GroupingExp(grouping_exp) => grouping_exp.interpret(environment),
            Node::UnaryExp(unary_exp) => unary_exp.interpret(environment),
            Node::BinaryExp(binary_exp) => binary_exp.interpret(environment),
            Node::LogicalExp(logical_exp) => logical_exp.interpret(environment),
            Node::AssignmentExp(assignment_exp) => assignment_exp.interpret(environment),
            Node::CallExp(call_exp) => call_exp.interpret(environment),
            Node::GetExp(get_exp) => get_exp.interpret(environment),
            Node::SetExp(set_exp) => set_exp.interpret(environment),
            Node::ThisExp(this_exp) => this_exp.interpret(environment),
            Node::SuperExp(super_exp) => super_exp.interpret(environment),

            // Statements
            Node::EmptyStm(empty_stm) => empty_stm.interpret(environment),
            Node::PrintStm(print_stm) => print_stm.interpret(environment),
            Node::BlockStm(block_stm) => block_stm.interpret(environment),
            Node::FuncBodyStm(func_body_stm) => func_body_stm.interpret(environment),
            Node::IfElseStm(if_else_stm) => if_else_stm.interpret(environment),
            Node::WhileStm(while_stm) => while_stm.interpret(environment),
            Node::ExpressionStm(expression_stm) => expression_stm.interpret(environment),
            Node::ReturnStm(return_stm) => return_stm.interpret(environment),
        }
    }
}
