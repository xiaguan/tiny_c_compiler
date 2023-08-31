use crate::stream::{BasicStream, Stream};

// use simplel logger to print log
use log::{debug, error, info};

#[derive(Debug, PartialEq)]
pub enum KeywordType {
    Mul,
    Div,
    Add,
    Sub,
    Lbracket,
    Rbracket,
}

impl KeywordType {
    // construct a keyword type from a char
    pub fn from_char(c: char) -> KeywordType {
        match c {
            '*' => KeywordType::Mul,
            '/' => KeywordType::Div,
            '+' => KeywordType::Add,
            '-' => KeywordType::Sub,
            '(' => KeywordType::Lbracket,
            ')' => KeywordType::Rbracket,
            _ => {
                error!("from_char: invalid char: {}", c);
                panic!("from_char: invalid char: {}", c);
            }
        }
    }
}

/// Token has three types
/// 1. keyword
/// 2. number
/// 3. eof (end of file)
#[derive(Debug, PartialEq)]
pub enum Token {
    /// the token is a keyword in C
    Keyword(KeywordType),
    /// the token is a number
    Number(i64),
    /// the token is a variable's name
    Var(String),
    /// end of file
    Eof,
    /// the token's init state ,undefined
    Unknown,
}

impl Token {
    /// If the token is a number, return the number
    /// otherwise ,panic
    pub fn get_number(&self) -> i64 {
        match self {
            Token::Number(number) => *number,
            _ => {
                error!("get_number: token is not a number");
                panic!("get_number: token is not a number");
            }
        }
    }

    /// return true if the token is eof  
    pub fn is_eof(&self) -> bool {
        match self {
            Token::Eof => true,
            Token::Unknown => {
                error!("is_eof() : the token is Unknown");
                panic!("is_eof() : the token is Unknown");
            }
            _ => false,
        }
    }

    /// if the token is a keyword, return the keyword
    /// otherwise, panic
    pub fn get_keyword(&self) -> &KeywordType {
        match self {
            Token::Keyword(keyword) => keyword,
            _ => {
                panic!("get_keyword: token is not a keyword: {:?}", self);
            }
        }
    }
}

/// basic scanner trait
pub trait Scanner {
    /// get the next token from the Scanner
    fn next_token(&mut self) -> Token;
}

#[derive(Debug)]
/// easy scanner for test
pub struct TinyCScanner {
    string_stream: BasicStream,
    current_buffer: Option<String>,
    cursor: usize,
}

/// This helper method parses a number token from the buffer.
fn parse_number_token(buffer: &str, index: &mut usize) -> Token {
    let start = *index;
    let mut end = start;
    // Range the str get the number end position
    for c in buffer[*index..].chars() {
        if c.is_ascii_digit() {
            end += 1;
        } else {
            break;
        }
    }
    let number: i64 = buffer[start..end].parse().unwrap();
    *index = end;
    Token::Number(number)
}

/// This helper method parses a keyword token from the buffer.
fn parse_keyword_token(c: char, index: &mut usize) -> Token {
    info!("Recognized a keyword char: {}", c);
    *index += 1;
    Token::Keyword(KeywordType::from_char(c))
}

impl TinyCScanner {
    /// This method generates the next token from the current buffer.
    /// It recognizes ASCII digits, ASCII whitespaces, and special keyword characters.
    ///
    /// # Panics
    ///
    /// The method will panic if it encounters an invalid character.
    fn make_next_token(&mut self) -> Token {
        let buffer = self.current_buffer.as_mut().unwrap();

        // Start with EOF token, change if another token is recognized
        let mut token = Token::Eof;
        let mut index = self.cursor;

        if index >= buffer.len() {
            return Token::Eof;
        }

        for c in buffer[index..].chars() {
            match c {
                _ if c.is_ascii_digit() => {
                    token = parse_number_token(buffer, &mut index);
                    break;
                }
                _ if c.is_ascii_whitespace() => {
                    info!("Skipping whitespace");
                    index += 1;
                }
                '+' | '-' | '*' | '/' | '(' | ')' => {
                    token = parse_keyword_token(c, &mut index);
                    break;
                }
                _ => panic!("TinyCScanner: Invalid char: {}", c),
            }
        }

        self.cursor = index;
        token
    }

    /// new
    pub fn new(string_stream: BasicStream) -> TinyCScanner {
        TinyCScanner {
            string_stream,
            current_buffer: None,
            cursor: 0,
        }
    }
}

// impl Scanner
impl Scanner for TinyCScanner {
    fn next_token(&mut self) -> Token {
        // get the current buffer
        if self.current_buffer.is_none() {
            self.current_buffer = self.string_stream.next();
            if self.current_buffer.is_none() {
                return Token::Eof;
            } else {
                info!("get a new buffer");
            }
        }
        let token = self.make_next_token();
        debug!("scanner next token: {:?}", token);
        token
    }
}

// test
#[cfg(test)]
mod tests {
    use super::*;

    fn create_scanner(string: String) -> TinyCScanner {
        TinyCScanner::new(BasicStream::new_with_string(string))
    }

    fn assert_next_number(scanner: &mut TinyCScanner, number: i64) {
        let token = scanner.next_token();
        assert_eq!(token, Token::Number(number));
    }

    fn assert_next_keyword(scanner: &mut TinyCScanner, keyword: KeywordType) {
        let token = scanner.next_token();
        assert_eq!(token, Token::Keyword(keyword));
    }

    #[test]
    fn test_scanner_eof() {
        let mut scanner = create_scanner("".to_string());
        assert_eq!(scanner.next_token(), Token::Eof);
    }

    #[test]
    fn test_scanner_parse_number() {
        let mut scanner = create_scanner("1234".to_owned());
        assert_next_number(&mut scanner, 1234)
    }

    #[test]
    fn test_sacnner_parse_keyword() {
        let mut scanner = create_scanner("+-*/()".to_owned());
        assert_next_keyword(&mut scanner, KeywordType::Add);
        assert_next_keyword(&mut scanner, KeywordType::Sub);
        assert_next_keyword(&mut scanner, KeywordType::Mul);
        assert_next_keyword(&mut scanner, KeywordType::Div);
        assert_next_keyword(&mut scanner, KeywordType::Lbracket);
        assert_next_keyword(&mut scanner, KeywordType::Rbracket);
        assert_eq!(scanner.next_token(), Token::Eof);
    }

    #[test]
    fn test_scanner_parse_number_and_keyword() {
        let mut scanner = create_scanner("123+456/789*0".to_owned());
        assert_next_number(&mut scanner, 123);
        assert_next_keyword(&mut scanner, KeywordType::Add);
        assert_next_number(&mut scanner, 456);
        assert_next_keyword(&mut scanner, KeywordType::Div);
        assert_next_number(&mut scanner, 789);
        assert_next_keyword(&mut scanner, KeywordType::Mul);
        assert_next_number(&mut scanner, 0);
        assert_eq!(scanner.next_token(), Token::Eof);
    }
}
