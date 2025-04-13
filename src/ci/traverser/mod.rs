mod traverser;

use std::{cell::RefCell, rc::Rc};

pub use traverser::{Traverse, TraverseResult, Traverser};

use super::Node;

impl<'de> Traverse<'de> for Node<'de> {
    #[rustfmt::skip]
    fn traverse(&self, traverser: Rc<RefCell<super::Traverser<'de>>>) -> TraverseResult {
        match self {
            // Declarations
            Node::VarDecl(var_decl) => var_decl.traverse(traverser),
            Node::FunctionDecl(function_decl) => function_decl.traverse(traverser),
            Node::ClassMethodDecl(class_method_decl) => class_method_decl.traverse(traverser),
            Node::ClassDecl(class_decl) => class_decl.traverse(traverser),
            Node::SuperClassDecl(super_class_decl) => super_class_decl.traverse(traverser),

            // Expressions
            Node::NilExp(nil_exp) => nil_exp.traverse(traverser),
            Node::LiteralExp(literal_exp) => literal_exp.traverse(traverser),
            Node::GroupingExp(grouping_exp) => grouping_exp.traverse(traverser),
            Node::UnaryExp(unary_exp) => unary_exp.traverse(traverser),
            Node::BinaryExp(binary_exp) => binary_exp.traverse(traverser),
            Node::LogicalExp(logical_exp) => logical_exp.traverse(traverser),
            Node::AssignmentExp(assignment_exp) => assignment_exp.traverse(traverser),
            Node::CallExp(call_exp) => call_exp.traverse(traverser),
            Node::GetExp(get_exp) => get_exp.traverse(traverser),
            Node::SetExp(set_exp) => set_exp.traverse(traverser),
            Node::ThisExp(this_exp) => this_exp.traverse(traverser),
            Node::SuperExp(super_exp) => super_exp.traverse(traverser),

            // Statements
            Node::EmptyStm(empty_stm) => empty_stm.traverse(traverser),
            Node::PrintStm(print_stm) => print_stm.traverse(traverser),
            Node::BlockStm(block_stm) => block_stm.traverse(traverser),
            Node::FuncBodyStm(func_body_stm) => func_body_stm.traverse(traverser),
            Node::IfElseStm(if_else_stm) => if_else_stm.traverse(traverser),
            Node::WhileStm(while_stm) => while_stm.traverse(traverser),
            Node::ExpressionStm(expression_stm) => expression_stm.traverse(traverser),
            Node::ReturnStm(return_stm) => return_stm.traverse(traverser),
        }
    }
}
