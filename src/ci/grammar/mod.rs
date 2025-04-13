pub(crate) mod debuge;
pub(crate) mod declaration;
pub(crate) mod expression;
pub(crate) mod parser;
pub(crate) mod statement;

pub use debuge::Debuge;
pub use parser::Parser;

use declaration::*;
use expression::*;
use statement::*;

#[rustfmt::skip] 
pub enum Node<'de> {
    // Declarations
    VarDecl(VarDecl<'de>),
    FunctionDecl(FunctionDecl<'de>),
    ClassMethodDecl(ClassMethodDecl<'de>),
    ClassDecl(ClassDecl<'de>),
    SuperClassDecl(SuperClassDecl<'de>),

    // Expressions
    NilExp(NilExp),
    LiteralExp(LiteralExp<'de>),
    GroupingExp(GroupingExp<'de>),
    UnaryExp(UnaryExp<'de>),
    BinaryExp(BinaryExp<'de>),
    LogicalExp(LogicalExp<'de>),
    AssignmentExp(AssignmentExp<'de>),
    CallExp(CallExp<'de>),
    GetExp(GetExp<'de>),
    SetExp(SetExp<'de>),
    ThisExp(ThisExp<'de>),
    SuperExp(SuperExp<'de>),

    // Statements
    EmptyStm(EmptyStm),
    PrintStm(PrintStm<'de>),
    BlockStm(BlockStm<'de>),
    FuncBodyStm(FuncBodyStm<'de>),
    IfElseStm(IfElseStm<'de>),
    WhileStm(WhileStm<'de>),
    ExpressionStm(ExpressionStm<'de>),
    ReturnStm(ReturnStm<'de>),
}

impl<'de> Debuge for Node<'de> {
    #[rustfmt::skip]
    fn print(&self) -> String {
        match self {
            // Declarations
            Node::VarDecl(var_decl) => var_decl.print(),
            Node::FunctionDecl(function_decl) => function_decl.print(),
            Node::ClassMethodDecl(class_method_decl) => class_method_decl.print(),
            Node::ClassDecl(class_decl) => class_decl.print(),
            Node::SuperClassDecl(super_class_decl) => super_class_decl.print(),

            // Expressions
            Node::NilExp(nil_exp) => nil_exp.print(),
            Node::LiteralExp(literal_exp) => literal_exp.print(),
            Node::GroupingExp(grouping_exp) => grouping_exp.print(),
            Node::UnaryExp(unary_exp) => unary_exp.print(),
            Node::BinaryExp(binary_exp) => binary_exp.print(),
            Node::LogicalExp(logical_exp) => logical_exp.print(),
            Node::AssignmentExp(assignment_exp) => assignment_exp.print(),
            Node::CallExp(call_exp) => call_exp.print(),
            Node::GetExp(get_exp) => get_exp.print(),
            Node::SetExp(set_exp) => set_exp.print(),
            Node::ThisExp(this_exp) => this_exp.print(),
            Node::SuperExp(super_exp) => super_exp.print(),

            // Statements
            Node::EmptyStm(empty_stm) => empty_stm.print(),
            Node::PrintStm(print_stm) => print_stm.print(),
            Node::BlockStm(block_stm) => block_stm.print(),
            Node::FuncBodyStm(func_body_stm) => func_body_stm.print(),
            Node::IfElseStm(if_else_stm) => if_else_stm.print(),
            Node::WhileStm(while_stm) => while_stm.print(),
            Node::ExpressionStm(expression_stm) => expression_stm.print(),
            Node::ReturnStm(return_stm) => return_stm.print(),
        }
    }
}
