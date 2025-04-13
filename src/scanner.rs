use miette::LabeledSpan;
use std::collections::HashMap;

#[derive(Debug, Clone)]
#[rustfmt::skip] 
#[allow(non_camel_case_types)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

    // One or two character tokens.
    BANG, BANG_EQUAL,
    EQUAL, EQUAL_EQUAL,
    GREATER, GREATER_EQUAL,
    LESS, LESS_EQUAL,

    // Literals.
    IDENTIFIER, STRING(String), NUMBER(f64),

    // Keywords.
    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

    EOF
}

#[derive(Debug, Clone)]
pub struct Token<'de> {
    pub token_type: TokenType,
    pub lexeme: &'de str,
    pub line: usize,
}

impl<'de> Token<'de> {
    pub fn new(token_type: TokenType, lexeme: &'de str, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            line,
        }
    }
}

impl<'de> std::fmt::Display for Token<'de> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let literal = match &self.token_type {
            TokenType::STRING(literal) => literal,
            TokenType::NUMBER(literal) => {
                if literal.fract() == 0.0 {
                    &format!("{:.1}", literal)
                } else {
                    &format!("{}", literal)
                }
            }
            _ => "null",
        };
        let type_name = format!("{:?}", self.token_type);
        let type_name = type_name.split('(').next().unwrap();
        write!(f, "{} {} {}", type_name, self.lexeme, literal)
    }
}

pub struct Scanner<'de> {
    source: &'de str,
    rest: &'de str,
    current: usize,
    line: usize,
    eof: bool,
}

impl<'de> Scanner<'de> {
    pub fn new(source: &'de str) -> Self {
        Scanner {
            source: source,
            rest: source,
            current: 0,
            line: 1,
            eof: false,
        }
    }

    fn get_keyword(&self, lemexe: &'de str) -> TokenType {
        //TODO: static
        [
            ("and", TokenType::AND),
            ("class", TokenType::CLASS),
            ("else", TokenType::ELSE),
            ("false", TokenType::FALSE),
            ("fun", TokenType::FUN),
            ("for", TokenType::FOR),
            ("if", TokenType::IF),
            ("nil", TokenType::NIL),
            ("or", TokenType::OR),
            ("print", TokenType::PRINT),
            ("return", TokenType::RETURN),
            ("super", TokenType::SUPER),
            ("this", TokenType::THIS),
            ("true", TokenType::TRUE),
            ("var", TokenType::VAR),
            ("while", TokenType::WHILE),
        ]
        .into_iter()
        .collect::<HashMap<&'static str, TokenType>>()
        .get(lemexe)
        .map(|x| x.clone())
        .unwrap_or(TokenType::IDENTIFIER)
    }

    fn advance_n(&mut self, n: usize) -> &'de str {
        assert!(n >= 1);

        let mut chars = self.rest.chars();
        let mut bytes_n = 0;
        for _ in 0..n {
            let c = chars.next().unwrap();
            bytes_n += c.len_utf8();
        }

        let lexeme = &self.rest[0..bytes_n];
        self.rest = &self.rest[bytes_n..];
        self.current += n;

        lexeme
    }

    fn peek_rest_at(&self, pos: usize) -> Option<char> {
        self.rest.chars().nth(pos)
    }

    fn scan_token(&mut self) -> Option<miette::Result<Token<'de>, miette::Error>> {
        fn token<'de>(
            token_type: TokenType,
            lexeme: &'de str,
            line: usize,
        ) -> Option<miette::Result<Token<'de>, miette::Error>> {
            Some(Ok(Token::<'de>::new(token_type, lexeme, line)))
        }

        'scan_loop: loop {
            let cur = if let Some(cur) = self.peek_rest_at(0) {
                cur
            } else {
                return None;
            };

            match cur {
                // Meaningless characters.
                ' ' | '\r' | '\t' => {
                    let _ = self.advance_n(1);
                }
                '\n' => {
                    self.line += 1;
                    let _ = self.advance_n(1);
                }
                // Single-character tokens.
                '(' => return token(TokenType::LEFT_PAREN, self.advance_n(1), self.line),
                ')' => return token(TokenType::RIGHT_PAREN, self.advance_n(1), self.line),
                '{' => return token(TokenType::LEFT_BRACE, self.advance_n(1), self.line),
                '}' => return token(TokenType::RIGHT_BRACE, self.advance_n(1), self.line),
                ',' => return token(TokenType::COMMA, self.advance_n(1), self.line),
                '.' => return token(TokenType::DOT, self.advance_n(1), self.line),
                '-' => return token(TokenType::MINUS, self.advance_n(1), self.line),
                '+' => return token(TokenType::PLUS, self.advance_n(1), self.line),
                ';' => return token(TokenType::SEMICOLON, self.advance_n(1), self.line),
                '*' => return token(TokenType::STAR, self.advance_n(1), self.line),
                '/' => match self.peek_rest_at(1) {
                    Some(next) if next == '/' => loop {
                        match self.peek_rest_at(0) {
                            Some(cur) if cur == '\n' => {
                                continue 'scan_loop;
                            }
                            Some(_) => {
                                // Comment content
                                let _ = self.advance_n(1);
                            }
                            None => continue 'scan_loop,
                        }
                    },
                    _ => return token(TokenType::SLASH, self.advance_n(1), self.line),
                },
                // One or two character tokens.
                '=' => match self.peek_rest_at(1) {
                    Some(next) if next == '=' => {
                        return token(TokenType::EQUAL_EQUAL, self.advance_n(2), self.line);
                    }
                    _ => return token(TokenType::EQUAL, self.advance_n(1), self.line),
                },
                '!' => match self.peek_rest_at(1) {
                    Some(next) if next == '=' => {
                        return token(TokenType::BANG_EQUAL, self.advance_n(2), self.line)
                    }
                    _ => return token(TokenType::BANG, self.advance_n(1), self.line),
                },
                '<' => match self.peek_rest_at(1) {
                    Some(next) if next == '=' => {
                        return token(TokenType::LESS_EQUAL, self.advance_n(2), self.line)
                    }
                    _ => return token(TokenType::LESS, self.advance_n(1), self.line),
                },
                '>' => match self.peek_rest_at(1) {
                    Some(next) if next == '=' => {
                        return token(TokenType::GREATER_EQUAL, self.advance_n(2), self.line)
                    }
                    _ => return token(TokenType::GREATER, self.advance_n(1), self.line),
                },
                // Literals.
                '\"' => {
                    let mut cur_len = 1;

                    loop {
                        match self.peek_rest_at(cur_len) {
                            Some(cur) if cur == '\n' => {
                                self.line += 1;
                                cur_len += 1;
                            }
                            Some(cur) if cur == '\"' => {
                                cur_len += 1;

                                let lexeme = self.advance_n(cur_len);
                                // Can cheat with trim
                                let literal = lexeme
                                    .chars()
                                    .skip(1)
                                    .take_while(|&c| c != '\"')
                                    .collect::<String>();

                                return token(TokenType::STRING(literal), lexeme, self.line);
                            }
                            Some(_) => {
                                cur_len += 1;
                            }
                            None => {
                                let _ = self.advance_n(cur_len);
                                let line = self.line;

                                return Some(Err(miette::miette! {
                                    labels = vec![LabeledSpan::at_offset(self.current, "here")],
                                    "[line {line}] Error: Unterminated string.",
                                }
                                .with_source_code(self.source.to_string())));
                            }
                        }
                    }
                }
                '0'..='9' => {
                    let mut cur_len = 0;
                    let mut seen_dot = false;

                    fn token_number<'de>(
                        lexeme: &'de str,
                        line: usize,
                    ) -> Option<miette::Result<Token<'de>, miette::Error>> {
                        let literal = lexeme.parse::<f64>().unwrap();

                        return token(TokenType::NUMBER(literal), lexeme, line);
                    }

                    loop {
                        match self.peek_rest_at(cur_len) {
                            Some(c) if c.is_digit(10) => {
                                cur_len += 1;
                            }
                            Some(c) if c == '.' && !seen_dot => {
                                seen_dot = true;

                                match self.peek_rest_at(cur_len + 1) {
                                    Some(next_c) if next_c.is_digit(10) => {
                                        cur_len += 1;
                                    }
                                    _ => return token_number(self.advance_n(cur_len), self.line),
                                };
                            }
                            _ => return token_number(self.advance_n(cur_len), self.line),
                        }
                    }
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut cur_len = 0;

                    loop {
                        match self.peek_rest_at(cur_len) {
                            Some(c) if c.is_alphanumeric() || c == '_' => {
                                cur_len += 1;
                            }
                            _ => {
                                let lexeme = self.advance_n(cur_len);
                                return token(self.get_keyword(lexeme), lexeme, self.line);
                            }
                        }
                    }
                }
                lexeme @ _ => {
                    let _ = self.advance_n(1);
                    let line = self.line;

                    return Some(Err(miette::miette! {
                        labels = vec![LabeledSpan::at_offset(self.current, "here")],
                        "[line {line}] Error: Unexpected character: {lexeme}",
                    }
                    .with_source_code(self.source.to_string())));
                }
            }
        }
    }
}

impl<'de> Iterator for Scanner<'de> {
    type Item = Result<Token<'de>, miette::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.scan_token();
        if token.is_some() {
            return token;
        } else {
            if !self.eof {
                self.eof = true;

                return Some(Ok(Token::new(TokenType::EOF, "", self.line)));
            } else {
                return None;
            }
        }
    }
}
