use crate::string_stream::{DoubleBufferStringStream, StreamBuffer, StringStream};

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
                error!("get_keyword: token is not a keyword");
                panic!("get_keyword: token is not a keyword");
            }
        }
    }
}

/// basic scanner trait
pub trait Scanner {
    /// get teh next token from the Scanner
    fn next_token(&mut self) -> Token;
}

#[derive(Debug)]
/// easy scanner for test
pub struct TinyCScanner {
    string_stream: DoubleBufferStringStream,
    current_buffer: Option<StreamBuffer>,
}

// return first number and the index of the first non-digit char
// if the string is empty, return (0,0)
pub(crate) fn strol(string: &[char], index: &mut usize) -> u64 {
    let mut number = 0;

    if string.is_empty() {
        error!("strol: string is empty");
        panic!("strol: string is empty");
    }

    while *index < string.len() {
        let c = string[*index];
        if c.is_ascii_digit() {
            number = number * 10 + (c as u64 - '0' as u64);
            *index += 1;
        } else {
            break;
        }
    }
    number
}

/// This helper method parses a number token from the buffer.
fn parse_number_token(buffer: &StreamBuffer, index: &mut usize) -> Token {
    let number_start = *index;
    let number = strol(&buffer.buffer, index);
    info!(
        "Parsed \"{}\" into number {}",
        &buffer.buffer[number_start..*index]
            .iter()
            .collect::<String>(),
        number
    );
    Token::Number(number as i64)
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

        info!(
            "Buffer content count: {}, Read index: {}",
            buffer.count, buffer.read_index
        );

        // Start with EOF token, change if another token is recognized
        let mut token = Token::Eof;
        let mut index = buffer.read_index;

        for &c in &buffer.buffer[index..buffer.count] {
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

        buffer.read_index = index;
        token
    }

    /// new
    pub fn new(string_stream: DoubleBufferStringStream) -> TinyCScanner {
        TinyCScanner {
            string_stream,
            current_buffer: None,
        }
    }
}

// impl Scanner
impl Scanner for TinyCScanner {
    fn next_token(&mut self) -> Token {
        // get the current buffer
        if self.current_buffer.is_none() {
            self.current_buffer = self.string_stream.next_buffer();
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

    use env_logger::Builder;
    use log::LevelFilter;

    #[test]
    fn test_scanner_parse_number() {
        let mut builder = Builder::from_default_env();

        builder.filter(None, LevelFilter::Info).try_init();
        use super::*;
        let mut scanner = TinyCScanner::new(DoubleBufferStringStream::new_with_string(
            "1234".to_string(),
        ));
        let token = scanner.next_token();
        assert_eq!(token.get_number(), 1234);
    }

    #[test]
    fn test_sacnner_parse_keyword() {
        let mut builder = Builder::from_default_env();

        builder.filter(None, LevelFilter::Info).try_init();
        use super::*;
        let mut scanner =
            TinyCScanner::new(DoubleBufferStringStream::new_with_string("+".to_string()));
        let token = scanner.next_token();
        assert_eq!(*token.get_keyword(), KeywordType::Add);
    }

    #[test]
    fn test_scanner_parse_number_and_keyword() {
        let mut builder = Builder::from_default_env();

        builder.filter(None, LevelFilter::Info).try_init();
        use super::*;
        let mut scanner = TinyCScanner::new(DoubleBufferStringStream::new_with_string(
            "1234 +".to_string(),
        ));
        let token = scanner.next_token();
        assert_eq!(token.get_number(), 1234);
        let token = scanner.next_token();
        assert_eq!(*token.get_keyword(), KeywordType::Add);
    }

    #[test]
    fn test_scanner_parse_expr() {
        let mut builder = Builder::from_default_env();

        builder.filter(None, LevelFilter::Info).try_init();

        use super::*;
        let mut scanner = TinyCScanner::new(DoubleBufferStringStream::new_with_string(
            "1234 + 1234".to_string(),
        ));
        let token = scanner.next_token();
        assert_eq!(token.get_number(), 1234);
        let token = scanner.next_token();
        assert_eq!(*token.get_keyword(), KeywordType::Add);
        let token = scanner.next_token();
        assert_eq!(token.get_number(), 1234);
    }
}
