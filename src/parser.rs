// What a parser does is to parse the tokens and provide code generator the AST.
// so it's input is tokens
// and it's output is AST

// Our goal is to parse a expression
// The priority of the operators is:
// 1. ()
// 2. * /
// 3. + -

use crate::ast::{BinaryOpType, Node};
use crate::scanner::{KeywordType, Scanner, TinyCScanner, Token};

use log::{debug, info};

/// `TinyCParser` is responsible for parsing tokens and providing the AST.
pub struct TinyCParser {
    scanner: TinyCScanner,
    current_token: Token,
}

impl TinyCParser {
    /// Constructs a new `TinyCParser`
    pub fn new(mut scanner: TinyCScanner) -> TinyCParser {
        let current_token = scanner.next_token();
        TinyCParser {
            scanner,
            current_token,
        }
    }

    /// Returns the current token
    pub fn get_current_token(&self) -> &Token {
        &self.current_token
    }

    /// Updates the current token to the next token from the scanner
    fn next(&mut self) {
        self.current_token = self.scanner.next_token();
    }

    /// Parses an expression according to the rule: expr = mul ('+' mul | '-' mul)*
    pub fn expr(&mut self) -> Node {
        let mut node = self.mul();
        debug!(" expr: {:?}", node);
        loop {
            match self.current_token {
                Token::Keyword(ref keyword) => match keyword {
                    KeywordType::Add => {
                        info!(" expr construct a add operation left node: {:?}", node);
                        self.next();
                        node = Node::from_binop(BinaryOpType::Add, node, self.mul());
                    }
                    KeywordType::Sub => {
                        info!(" expr construct a sub operation left node: {:?}", node);
                        self.next();
                        node = Node::from_binop(BinaryOpType::Sub, node, self.mul());
                    }
                    _ => {
                        return node;
                    }
                },
                _ => {
                    return node;
                }
            }
        }
    }

    /// Parses a mul expression according to the rule: mul = primary ('*' primary | '/' primary)*
    fn mul(&mut self) -> Node {
        let mut node = self.primary();
        loop {
            match self.current_token {
                Token::Keyword(ref keyword) => match keyword {
                    KeywordType::Mul => {
                        info!(" mul construct a mul operation left node: {:?}", node);
                        self.next();
                        node = Node::from_binop(BinaryOpType::Mul, node, self.primary());
                    }
                    KeywordType::Div => {
                        info!(" mul construct a div operation left node: {:?}", node);
                        self.next();
                        node = Node::from_binop(BinaryOpType::Div, node, self.primary());
                    }
                    _ => {
                        return node;
                    }
                },
                _ => {
                    return node;
                }
            }
        }
    }

    /// Parses a primary expression, which is a number or a bracketed expression.
    fn primary(&mut self) -> Node {
        match self.current_token {
            Token::Number(n) => {
                self.next();
                info!(" primary: get a number: {}", n);
                Node::from_num(n)
            }
            Token::Keyword(KeywordType::Lbracket) => {
                debug!("primary: get a left bracket try to get an expr");
                self.next();
                let node = self.expr();
                match self.current_token {
                    Token::Keyword(ref keyword) => {
                        if *keyword == KeywordType::Rbracket {
                            // skip the right bracket
                            self.next();
                            node
                        } else {
                            panic!("primary: expect a right bracket");
                        }
                    }
                    _ => panic!("primary: expect a right bracket"),
                }
            }
            _ => panic!("primary: expect a number or a left bracket"),
        }
    }
}
