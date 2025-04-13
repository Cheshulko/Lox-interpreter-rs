use std::{iter::Peekable, process::exit, rc::Rc};

use crate::{Scanner, Token, TokenType};

use super::{declaration::*, expression::*, statement::*, Node};

macro_rules! ensure_consume_matches {
    ($scanner:expr, $( $pattern:pat ),* $(,)?) => {
        match $scanner.next().transpose() {
            $( Ok(Some(token @ Token {
                token_type: $pattern,
                ..
            })) => Result::<Token, anyhow::Error>::Ok(token), )*
            _ => {
                let expected_patterns = vec![$(stringify!($pattern)),*];
                anyhow::bail!(
                    "Unexpected token. Expected one of: {}",
                    expected_patterns.join(", ")
                )
            },
        }
    };
}

macro_rules! peek_matches {
    ($scanner:expr, $( $pattern:pat ),* $(,)?) => {
        if let Some(value) = $scanner.peek() {
            match value
                .as_ref()
                .map_err(|e| anyhow::anyhow! { e.to_string() })? {
                $( Token {
                    token_type: $pattern,
                    ..
                } => Result::<bool, anyhow::Error>::Ok(true), )*
                _ => Result::<bool, anyhow::Error>::Ok(false),
            }
        } else {
            Result::<bool, anyhow::Error>::Ok(false)
        }
    };
}

macro_rules! consume_matches {
    ($scanner:expr, $( $pattern:pat ),* $(,)?) => {
        if let Ok(true) = peek_matches!($scanner, $( $pattern ),*) {
            Some(ensure_consume_matches!($scanner, $( $pattern ),*)?)
        } else {
            None
        }
    };
}

pub struct Parser<'de> {
    _source: &'de str,
    scanner: Peekable<Scanner<'de>>,
}

impl<'de> Parser<'de> {
    pub fn new(source: &'de str, scanner: Scanner<'de>) -> Parser<'de> {
        Parser {
            _source: source,
            scanner: scanner.peekable(),
        }
    }

    pub fn parse_statements(self) -> impl IntoIterator<Item = Box<Node<'de>>> {
        let mut statements = vec![];

        for statement in self {
            match statement {
                Ok(statement) => statements.push(statement),
                Err(error) => {
                    eprintln!("{error}");
                    exit(65);
                }
            }
        }

        statements
    }

    pub fn parse(&mut self) -> Option<Result<Box<Node<'de>>, anyhow::Error>> {
        while let Some(token) = self.scanner.peek() {
            match token
                .as_ref()
                .map_err(|e| anyhow::anyhow! { e.to_string() })
            {
                Ok(token) => {
                    if matches!(token.token_type, TokenType::EOF) {
                        return None;
                    }

                    if matches!(token.token_type, TokenType::CLASS) {
                        return Some(self.class_declaration());
                    }

                    if matches!(token.token_type, TokenType::FUN) {
                        return Some(self.fun_declaration());
                    }

                    if matches!(token.token_type, TokenType::VAR) {
                        return Some(self.var_declaration());
                    }

                    return Some(self.parse_statement());
                }
                Err(error) => return Some(Err(error)),
            }
        }

        unreachable!()
    }

    pub fn parse_statement(&mut self) -> Result<Box<Node<'de>>, anyhow::Error> {
        if let Some(token) = self.scanner.peek() {
            match token
                .as_ref()
                .map_err(|e| anyhow::anyhow! { e.to_string() })
            {
                Ok(token) => {
                    if matches!(token.token_type, TokenType::PRINT) {
                        return self.print_statement();
                    }

                    if matches!(token.token_type, TokenType::LEFT_BRACE) {
                        return self.bloc_statement();
                    }

                    if matches!(token.token_type, TokenType::IF) {
                        return self.if_else_statement();
                    }

                    if matches!(token.token_type, TokenType::WHILE) {
                        return self.while_statement();
                    }

                    if matches!(token.token_type, TokenType::FOR) {
                        return self.for_statement();
                    }

                    if matches!(token.token_type, TokenType::RETURN) {
                        return self.return_statement();
                    }

                    return self.expression_statement();
                }
                Err(error) => return Err(error),
            }
        }

        unreachable!()
    }

    pub fn parse_expression(&mut self) -> Result<Box<Node<'de>>, anyhow::Error> {
        return self.expression();
    }

    fn class_declaration(&mut self) -> Result<Box<Node<'de>>, anyhow::Error> {
        let _ = ensure_consume_matches!(self.scanner, TokenType::CLASS)?;

        let name = ensure_consume_matches!(self.scanner, TokenType::IDENTIFIER)?;

        let super_class = if let Some(_) = consume_matches!(self.scanner, TokenType::LESS) {
            let name = ensure_consume_matches!(self.scanner, TokenType::IDENTIFIER)?;

            Some(SuperClassDecl { name })
        } else {
            None
        };

        let _ = ensure_consume_matches!(self.scanner, TokenType::LEFT_BRACE)?;

        let mut methods = vec![];
        while !peek_matches!(self.scanner, TokenType::RIGHT_BRACE | TokenType::EOF)? {
            let method = self.class_method_declaration()?;
            methods.push(method);
        }

        let _ = ensure_consume_matches!(self.scanner, TokenType::RIGHT_BRACE)?;

        return Ok(Box::new(Node::ClassDecl(ClassDecl {
            name,
            super_class,
            methods,
        })));
    }

    fn class_method_declaration(&mut self) -> Result<Box<Node<'de>>, anyhow::Error> {
        let name = ensure_consume_matches!(self.scanner, TokenType::IDENTIFIER)?;

        let _ = ensure_consume_matches!(self.scanner, TokenType::LEFT_PAREN)?;
        let mut parameters = vec![];
        if !peek_matches!(self.scanner, TokenType::RIGHT_PAREN)? {
            loop {
                // TODO: Check max arguments count
                // if args.len() >= 255 {
                //     panic!()
                // }
                let parameter = ensure_consume_matches!(self.scanner, TokenType::IDENTIFIER)?;
                parameters.push(parameter);

                if let Some(_) = consume_matches!(self.scanner, TokenType::COMMA) {
                } else {
                    break;
                }
            }
        }
        let _ = ensure_consume_matches!(self.scanner, TokenType::RIGHT_PAREN)?;

        let body = self.func_body_statement()?;
        let body = Rc::from(body);

        return Ok(Box::new(Node::ClassMethodDecl(ClassMethodDecl {
            name,
            parameters,
            body,
        })));
    }

    fn fun_declaration(&mut self) -> Result<Box<Node<'de>>, anyhow::Error> {
        ensure_consume_matches!(self.scanner, TokenType::FUN)?;

        let name = ensure_consume_matches!(self.scanner, TokenType::IDENTIFIER)?;

        let _ = ensure_consume_matches!(self.scanner, TokenType::LEFT_PAREN)?;
        let mut parameters = vec![];
        if !peek_matches!(self.scanner, TokenType::RIGHT_PAREN)? {
            loop {
                // TODO: Check max arguments count
                // if args.len() >= 255 {
                //     panic!()
                // }
                let parameter = ensure_consume_matches!(self.scanner, TokenType::IDENTIFIER)?;
                parameters.push(parameter);

                if let Some(_) = consume_matches!(self.scanner, TokenType::COMMA) {
                } else {
                    break;
                }
            }
        }
        let _ = ensure_consume_matches!(self.scanner, TokenType::RIGHT_PAREN)?;

        let body = self.func_body_statement()?;
        let body = Rc::from(body);

        return Ok(Box::new(Node::FunctionDecl(FunctionDecl {
            name,
            parameters,
            body,
        })));
    }

    fn var_declaration(&mut self) -> Result<Box<Node<'de>>, anyhow::Error> {
        let _ = ensure_consume_matches!(self.scanner, TokenType::VAR)?;

        let name = ensure_consume_matches!(self.scanner, TokenType::IDENTIFIER)?;

        if let Some(_) = consume_matches!(self.scanner, TokenType::EQUAL) {
            let initializer = self.expression()?;
            let _ = ensure_consume_matches!(self.scanner, TokenType::SEMICOLON)?;

            return Ok(Box::new(Node::VarDecl(VarDecl {
                name,
                initializer: Some(initializer),
            })));
        } else {
            let _ = ensure_consume_matches!(self.scanner, TokenType::SEMICOLON)?;

            return Ok(Box::new(Node::VarDecl(VarDecl {
                name,
                initializer: None,
            })));
        }
    }

    fn print_statement(&mut self) -> Result<Box<Node<'de>>, anyhow::Error> {
        let _ = ensure_consume_matches!(self.scanner, TokenType::PRINT)?;
        let expression = self.expression()?;
        let _ = ensure_consume_matches!(self.scanner, TokenType::SEMICOLON)?;

        return Ok(Box::new(Node::PrintStm(PrintStm { expression })));
    }

    fn bloc_statement(&mut self) -> Result<Box<Node<'de>>, anyhow::Error> {
        let _ = ensure_consume_matches!(self.scanner, TokenType::LEFT_BRACE)?;

        let mut statements = vec![];
        while !peek_matches!(self.scanner, TokenType::RIGHT_BRACE)? {
            if let Some(statement) = self.parse() {
                let statement = statement?;
                statements.push(statement);
            } else {
                anyhow::bail! {"Unexpected end of the block with no right brace ending"};
            }
        }

        let _ = ensure_consume_matches!(self.scanner, TokenType::RIGHT_BRACE)?;

        return Ok(Box::new(Node::BlockStm(BlockStm { statements })));
    }

    fn func_body_statement(&mut self) -> Result<Box<Node<'de>>, anyhow::Error> {
        let _ = ensure_consume_matches!(self.scanner, TokenType::LEFT_BRACE)?;

        let mut statements = vec![];
        while !peek_matches!(self.scanner, TokenType::RIGHT_BRACE)? {
            if let Some(statement) = self.parse() {
                let statement = statement?;
                statements.push(statement);
            } else {
                anyhow::bail! {"Unexpected end of the block with no right brace ending"};
            }
        }

        let _ = ensure_consume_matches!(self.scanner, TokenType::RIGHT_BRACE)?;

        return Ok(Box::new(Node::FuncBodyStm(FuncBodyStm { statements })));
    }

    fn if_else_statement(&mut self) -> Result<Box<Node<'de>>, anyhow::Error> {
        let _ = ensure_consume_matches!(self.scanner, TokenType::IF)?;

        let _ = ensure_consume_matches!(self.scanner, TokenType::LEFT_PAREN)?;
        let condition = self.expression()?;
        let _ = ensure_consume_matches!(self.scanner, TokenType::RIGHT_PAREN)?;

        let then_branch = self.parse_statement()?;

        let else_branch = if let Some(_) = consume_matches!(self.scanner, TokenType::ELSE) {
            Some(self.parse_statement()?)
        } else {
            None
        };

        return Ok(Box::new(Node::IfElseStm(IfElseStm {
            condition,
            then_branch,
            else_branch,
        })));
    }

    fn while_statement(&mut self) -> Result<Box<Node<'de>>, anyhow::Error> {
        let _ = ensure_consume_matches!(self.scanner, TokenType::WHILE)?;

        let _ = ensure_consume_matches!(self.scanner, TokenType::LEFT_PAREN)?;
        let condition = self.expression()?;
        let _ = ensure_consume_matches!(self.scanner, TokenType::RIGHT_PAREN)?;

        let body = self.parse_statement()?;

        return Ok(Box::new(Node::WhileStm(WhileStm { condition, body })));
    }

    fn for_statement(&mut self) -> Result<Box<Node<'de>>, anyhow::Error> {
        let _ = ensure_consume_matches!(self.scanner, TokenType::FOR)?;
        let _ = ensure_consume_matches!(self.scanner, TokenType::LEFT_PAREN)?;

        /*
            for (var i = 0; i < 10; i = i + 1) {
                print i;
            }
            <=>
            {
                var i = 0;
                while (i < 10) {
                    print i;

                    i = i + 1;
                }
            }
        */

        let initializer = if let Some(_) = consume_matches!(self.scanner, TokenType::SEMICOLON) {
            Box::new(Node::EmptyStm(EmptyStm {}))
        } else {
            if peek_matches!(self.scanner, TokenType::VAR)? {
                self.var_declaration()?
            } else {
                self.expression_statement()?
            }
        };

        let condition = if let Some(_) = consume_matches!(self.scanner, TokenType::SEMICOLON) {
            // TODO: const `true` literal
            let true_const = {
                let true_const_token = Token::new(TokenType::TRUE, "", 42);

                Box::new(Node::LiteralExp(LiteralExp {
                    name: true_const_token,
                }))
            };

            true_const
        } else {
            let condition = self.expression()?;
            ensure_consume_matches!(self.scanner, TokenType::SEMICOLON)?;

            condition
        };

        let increment = if let Some(_) = consume_matches!(self.scanner, TokenType::RIGHT_PAREN) {
            Box::new(Node::EmptyStm(EmptyStm {}))
        } else {
            let increment = self.expression()?;
            ensure_consume_matches!(self.scanner, TokenType::RIGHT_PAREN)?;

            increment
        };

        let body = self.parse_statement()?;

        return Ok(Box::new(Node::BlockStm(BlockStm {
            statements: vec![
                initializer,
                Box::new(Node::WhileStm(WhileStm {
                    condition,
                    body: Box::new(Node::BlockStm(BlockStm {
                        statements: vec![body, increment],
                    })),
                })),
            ],
        })));
    }

    fn return_statement(&mut self) -> Result<Box<Node<'de>>, anyhow::Error> {
        let _ = ensure_consume_matches!(self.scanner, TokenType::RETURN)?;
        let result = if !peek_matches!(self.scanner, TokenType::SEMICOLON)? {
            self.expression()?
        } else {
            Box::new(Node::NilExp(NilExp {}))
        };
        let _ = ensure_consume_matches!(self.scanner, TokenType::SEMICOLON)?;

        return Ok(Box::new(Node::ReturnStm(ReturnStm { expression: result })));
    }

    fn expression_statement(&mut self) -> Result<Box<Node<'de>>, anyhow::Error> {
        let expression = self.expression()?;
        let _ = ensure_consume_matches!(self.scanner, TokenType::SEMICOLON)?;

        return Ok(Box::new(Node::ExpressionStm(ExpressionStm { expression })));
    }

    fn expression(&mut self) -> Result<Box<Node<'de>>, anyhow::Error> {
        return self.assignment();
    }

    fn assignment(&mut self) -> Result<Box<Node<'de>>, anyhow::Error> {
        let expression = self.or()?;

        return match expression.as_ref() {
            Node::GetExp(_) if peek_matches!(self.scanner, TokenType::EQUAL)? => {
                // Ok. That is a property setter
                let _ = ensure_consume_matches!(self.scanner, TokenType::EQUAL);

                let value = self.assignment()?;

                Ok(Box::new(Node::SetExp(SetExp {
                    get_exp: expression,
                    value,
                })))
            }
            Node::LiteralExp(literal) if peek_matches!(self.scanner, TokenType::EQUAL)? => {
                // Ok. That is an variable assignment
                let _ = ensure_consume_matches!(self.scanner, TokenType::EQUAL);

                let name = literal.name.clone();
                let value = self.assignment()?;

                Ok(Box::new(Node::AssignmentExp(AssignmentExp { name, value })))
            }
            _ => Ok(expression),
        };
    }

    fn or(&mut self) -> Result<Box<Node<'de>>, anyhow::Error> {
        let mut left = self.and()?;

        while let Some(operator) = consume_matches!(self.scanner, TokenType::OR) {
            let right = self.and()?;

            left = Box::new(Node::LogicalExp(LogicalExp {
                left,
                operator,
                right,
            }));
        }

        return Ok(left);
    }

    fn and(&mut self) -> Result<Box<Node<'de>>, anyhow::Error> {
        let mut left = self.equality()?;

        while let Some(operator) = consume_matches!(self.scanner, TokenType::AND) {
            let right = self.equality()?;

            left = Box::new(Node::LogicalExp(LogicalExp {
                left,
                operator,
                right,
            }));
        }

        return Ok(left);
    }

    fn equality(&mut self) -> Result<Box<Node<'de>>, anyhow::Error> {
        let mut left = self.comparison()?;

        while let Some(operator) =
            consume_matches!(self.scanner, TokenType::BANG_EQUAL | TokenType::EQUAL_EQUAL)
        {
            let right = self.comparison()?;

            left = Box::new(Node::BinaryExp(BinaryExp {
                left,
                operator,
                right,
            }));
        }

        return Ok(left);
    }

    fn comparison(&mut self) -> Result<Box<Node<'de>>, anyhow::Error> {
        let mut left = self.term()?;

        while let Some(operator) = consume_matches!(
            self.scanner,
            TokenType::GREATER | TokenType::GREATER_EQUAL | TokenType::LESS | TokenType::LESS_EQUAL
        ) {
            let right = self.term()?;

            left = Box::new(Node::BinaryExp(BinaryExp {
                left,
                operator,
                right,
            }));
        }

        return Ok(left);
    }

    fn term(&mut self) -> Result<Box<Node<'de>>, anyhow::Error> {
        let mut left = self.factor()?;

        while let Some(operator) =
            consume_matches!(self.scanner, TokenType::MINUS | TokenType::PLUS)
        {
            let right = self.factor()?;

            left = Box::new(Node::BinaryExp(BinaryExp {
                left,
                operator,
                right,
            }));
        }

        return Ok(left);
    }

    fn factor(&mut self) -> Result<Box<Node<'de>>, anyhow::Error> {
        let mut left = self.unary()?;

        while let Some(operator) =
            consume_matches!(self.scanner, TokenType::STAR | TokenType::SLASH)
        {
            let right = self.unary()?;

            left = Box::new(Node::BinaryExp(BinaryExp {
                left,
                operator,
                right,
            }));
        }

        return Ok(left);
    }

    fn unary(&mut self) -> Result<Box<Node<'de>>, anyhow::Error> {
        if peek_matches!(self.scanner, TokenType::EOF)? {
            anyhow::bail! {"Unexpected EOF"};
        }

        if let Some(operator) = consume_matches!(self.scanner, TokenType::BANG | TokenType::MINUS) {
            let right = self.unary()?;

            return Ok(Box::new(Node::UnaryExp(UnaryExp { operator, right })));
        } else {
            return self.call();
        }
    }

    fn call(&mut self) -> Result<Box<Node<'de>>, anyhow::Error> {
        let mut callee = self.primary()?;

        while peek_matches!(self.scanner, TokenType::LEFT_PAREN | TokenType::DOT)? {
            if peek_matches!(self.scanner, TokenType::LEFT_PAREN)? {
                let _ = ensure_consume_matches!(self.scanner, TokenType::LEFT_PAREN)?;
                callee = self.call_func(callee)?;
                let _ = ensure_consume_matches!(self.scanner, TokenType::RIGHT_PAREN)?;
            }

            if peek_matches!(self.scanner, TokenType::DOT)? {
                let _ = ensure_consume_matches!(self.scanner, TokenType::DOT)?;
                callee = self.get_property(callee)?;
            }
        }

        return Ok(callee);
    }

    fn call_func(&mut self, callee: Box<Node<'de>>) -> Result<Box<Node<'de>>, anyhow::Error> {
        let mut args = vec![];
        if !peek_matches!(self.scanner, TokenType::RIGHT_PAREN)? {
            loop {
                // TODO: Check max arguments count
                // if args.len() >= 255 {
                //     panic!()
                // }
                args.push(self.expression()?);
                if let Some(_) = consume_matches!(self.scanner, TokenType::COMMA) {
                } else {
                    break;
                }
            }
        }

        return Ok(Box::new(Node::CallExp(CallExp { callee, args })));
    }

    fn get_property(&mut self, callee: Box<Node<'de>>) -> Result<Box<Node<'de>>, anyhow::Error> {
        if let Some(name) = consume_matches!(self.scanner, TokenType::IDENTIFIER) {
            return Ok(Box::new(Node::GetExp(GetExp { callee, name })));
        } else {
            anyhow::bail! {"Expect property name after '.'."}
        }
    }

    fn primary(&mut self) -> Result<Box<Node<'de>>, anyhow::Error> {
        if peek_matches!(self.scanner, TokenType::EOF)? {
            anyhow::bail! {"Unexpected EOF"};
        }

        if let Some(name) = consume_matches!(
            self.scanner,
            TokenType::FALSE
                | TokenType::TRUE
                | TokenType::NIL
                | TokenType::STRING(_)
                | TokenType::NUMBER(_)
        ) {
            return Ok(Box::new(Node::LiteralExp(LiteralExp { name })));
        }

        if let Some(token) = consume_matches!(self.scanner, TokenType::THIS) {
            return Ok(Box::new(Node::ThisExp(ThisExp { token })));
        }

        if let Some(name) = consume_matches!(self.scanner, TokenType::IDENTIFIER) {
            return Ok(Box::new(Node::LiteralExp(LiteralExp { name })));
        }

        if let Some(token) = consume_matches!(self.scanner, TokenType::SUPER) {
            let _ = ensure_consume_matches!(self.scanner, TokenType::DOT);

            let method = ensure_consume_matches!(self.scanner, TokenType::IDENTIFIER)?;

            return Ok(Box::new(Node::SuperExp(SuperExp { token, method })));
        }

        if let Some(_) = consume_matches!(self.scanner, TokenType::LEFT_PAREN) {
            let inner = self.expression()?;

            if let Some(right) = self.scanner.next() {
                let right = match right {
                    Ok(ok) => ok,
                    Err(e) => anyhow::bail! {e.to_string()},
                };
                if !matches!(right.token_type, TokenType::RIGHT_PAREN) {
                    anyhow::bail! { Parser::error_at(right.line, right.lexeme, "Expect )") };
                }
            } else {
                anyhow::bail! {"Unexpected EOF"};
            }

            return Ok(Box::new(Node::GroupingExp(GroupingExp { inner })));
        }

        let token = self
            .scanner
            .next()
            .transpose()
            .expect("Checked at peek")
            .expect("Checked at peek");
        anyhow::bail! { Parser::error_at(token.line, token.lexeme, "Expect expression.") };
    }

    fn error_at(line: usize, lexeme: &str, message: &str) -> String {
        format!("[line {line}] Error at '{lexeme}': {message}'")
    }
}

impl<'de> Iterator for Parser<'de> {
    type Item = Result<Box<Node<'de>>, anyhow::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.parse()
    }
}
