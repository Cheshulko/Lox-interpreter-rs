use crate::TokenType;

use super::{declaration::*, expression::*, statement::*, Node};

pub trait Debuge {
    fn print(&self) -> String;

    fn parenthesize<'a>(
        &self,
        name: &str,
        exprs: impl IntoIterator<Item = &'a Node<'a>>,
    ) -> String {
        let mut s = String::new();
        s.push('(');
        s.push_str(name);

        for expr in exprs.into_iter() {
            s.push(' ');
            s.push_str(&expr.print());
        }
        s.push(')');

        s
    }
}

// Declarations
impl<'de> Debuge for VarDecl<'de> {
    fn print(&self) -> String {
        return self.parenthesize(
            &format!("var {name}", name = self.name),
            if let Some(initializer) = &self.initializer {
                vec![initializer.as_ref()]
            } else {
                vec![]
            },
        );
    }
}
impl<'de> Debuge for FunctionDecl<'de> {
    fn print(&self) -> String {
        return self.parenthesize(
            &format!("fun {name}", name = self.name),
            vec![self.body.as_ref()],
        );
    }
}
impl<'de> Debuge for ClassMethodDecl<'de> {
    fn print(&self) -> String {
        return self.parenthesize(
            &format!("method {name}", name = self.name),
            vec![self.body.as_ref()],
        );
    }
}
impl<'de> Debuge for ClassDecl<'de> {
    fn print(&self) -> String {
        return self.parenthesize(&format!("class {name}", name = self.name), vec![]);
    }
}
impl<'de> Debuge for SuperClassDecl<'de> {
    fn print(&self) -> String {
        return self.parenthesize(&format!("class {name}", name = self.name), vec![]);
    }
}

// Expressions
impl<'de> Debuge for NilExp {
    fn print(&self) -> String {
        return self.parenthesize("nil", vec![]);
    }
}
impl<'de> Debuge for LiteralExp<'de> {
    fn print(&self) -> String {
        match &self.name.token_type {
            TokenType::NIL => "nil".to_string(),
            TokenType::TRUE => "true".to_string(),
            TokenType::FALSE => "false".to_string(),
            TokenType::STRING(literal) => literal.to_string(),
            TokenType::NUMBER(literal) => {
                if literal.fract() == 0.0 {
                    format!("{:.1}", literal)
                } else {
                    format!("{}", literal)
                }
            }
            token => format!("{:?}", token),
        }
    }
}
impl<'de> Debuge for GroupingExp<'de> {
    fn print(&self) -> String {
        return self.parenthesize("group", vec![self.inner.as_ref()]);
    }
}
impl<'de> Debuge for UnaryExp<'de> {
    fn print(&self) -> String {
        return self.parenthesize(&self.operator.lexeme, vec![self.right.as_ref()]);
    }
}
impl<'de> Debuge for BinaryExp<'de> {
    fn print(&self) -> String {
        return self.parenthesize(
            &self.operator.lexeme,
            vec![self.left.as_ref(), self.right.as_ref()],
        );
    }
}
impl<'de> Debuge for LogicalExp<'de> {
    fn print(&self) -> String {
        return self.parenthesize(
            &self.operator.lexeme,
            vec![self.left.as_ref(), self.right.as_ref()],
        );
    }
}
impl<'de> Debuge for AssignmentExp<'de> {
    fn print(&self) -> String {
        return self.parenthesize(
            &format!("{name}", name = &self.name.lexeme),
            vec![self.value.as_ref()],
        );
    }
}
impl<'de> Debuge for CallExp<'de> {
    fn print(&self) -> String {
        return self.parenthesize(
            "fn",
            vec![self.callee.as_ref()]
                .into_iter()
                .chain(self.args.iter().map(|arg| arg.as_ref())),
        );
    }
}

impl<'de> Debuge for GetExp<'de> {
    fn print(&self) -> String {
        return self.parenthesize(
            &format!("get {}", self.name.lexeme),
            vec![self.callee.as_ref()],
        );
    }
}
impl<'de> Debuge for SetExp<'de> {
    fn print(&self) -> String {
        return self.parenthesize(
            &format!("set"),
            vec![self.get_exp.as_ref(), self.value.as_ref()],
        );
    }
}
impl<'de> Debuge for ThisExp<'de> {
    fn print(&self) -> String {
        return self.parenthesize(&format!("this"), vec![]);
    }
}
impl<'de> Debuge for SuperExp<'de> {
    fn print(&self) -> String {
        return self.parenthesize(&format!("super"), vec![]);
    }
}

// Statements
impl<'de> Debuge for EmptyStm {
    fn print(&self) -> String {
        String::new()
    }
}
impl<'de> Debuge for PrintStm<'de> {
    fn print(&self) -> String {
        return self.parenthesize("print", vec![self.expression.as_ref()]);
    }
}
impl<'de> Debuge for BlockStm<'de> {
    fn print(&self) -> String {
        return self.parenthesize(
            &format!("{{}}"),
            self.statements.iter().map(|statement| statement.as_ref()),
        );
    }
}
impl<'de> Debuge for FuncBodyStm<'de> {
    fn print(&self) -> String {
        return self.parenthesize(
            &format!("{{}}"),
            self.statements.iter().map(|statement| statement.as_ref()),
        );
    }
}
impl<'de> Debuge for IfElseStm<'de> {
    fn print(&self) -> String {
        let mut exprs = vec![self.condition.as_ref(), self.then_branch.as_ref()];
        if let Some(ref else_branch) = self.else_branch {
            exprs.push(else_branch.as_ref());
        }
        return self.parenthesize(&format!("if"), exprs);
    }
}
impl<'de> Debuge for WhileStm<'de> {
    fn print(&self) -> String {
        return self.parenthesize("while", vec![self.condition.as_ref(), self.body.as_ref()]);
    }
}
impl<'de> Debuge for ExpressionStm<'de> {
    fn print(&self) -> String {
        return self.parenthesize("none", vec![self.expression.as_ref()]);
    }
}
impl<'de> Debuge for ReturnStm<'de> {
    fn print(&self) -> String {
        return self.parenthesize("return", vec![self.expression.as_ref()]);
    }
}
