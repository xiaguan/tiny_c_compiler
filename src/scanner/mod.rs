use crate::string_stream::{DoubleBufferStringStream, StreamBuffer, StringStream};

// use simplel logger to print log
use log::{debug, error, info};

/// Token has three types
/// 1. keyword
/// 2. number
/// 3. eof (end of file)
#[derive(Debug, PartialEq)]
pub enum Token {
    /// the token is a keyword in C
    Keyword(String),
    /// the token is a number
    Number(i32),
    /// the token is a variable's name
    Var(String),
    /// end of file
    Eof,
    /// the token's init state ,undefined
    Unknown,
}

impl Token {
    pub(crate) fn new() -> Token {
        Token::Unknown
    }

    /// If the token is a number, return the number
    /// otherwise ,panic
    pub fn get_number(&self) -> i32 {
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
    pub fn get_keyword(&self) -> &String {
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
    fn next_token(&mut self) -> Option<Token>;
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

impl TinyCScanner {
    fn make_next_token(&mut self) -> Token {
        let buffer = self.current_buffer.as_mut().unwrap();
        // debug the buffer
        info!("buffer cnt : {} index {} ", buffer.count, buffer.read_index);
        let mut token = Token::Eof;
        let mut index = buffer.read_index;
        while index < buffer.count {
            let c = buffer.buffer[index];
            if c.is_ascii_digit() {
                // get the number
                let before_index = index;
                info!(
                    "current buf {}",
                    &buffer.buffer[before_index..].iter().collect::<String>()
                );
                let number = strol(&buffer.buffer, &mut index);
                // debug the number
                info!(
                    "parse from buffer: {} to {} str \"{}\" number {}",
                    before_index,
                    index,
                    &buffer.buffer[before_index..index]
                        .iter()
                        .collect::<String>(),
                    number
                );
                token = Token::Number(number as i32);
                break;
            } else if c.is_ascii_whitespace() {
                // skip whitespace
                index += 1;
                info!("skip whitespace");
            } else if c == '+' || c == '-' {
                info!("for preverse future, skip + or -");
                index += 1;
                token = Token::Keyword(c.to_string());
                break;
            } else {
                // error
                error!("TinyCScanner: invalid char: {}", c);
                panic!("TinyCScanner: invalid char: {}", c);
            }
        }
        // update the read index
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
    fn next_token(&mut self) -> Option<Token> {
        // get the current buffer
        if self.current_buffer.is_none() {
            self.current_buffer = self.string_stream.next_buffer();
            if self.current_buffer.is_none() {
                return None;
            } else {
                info!("get a new buffer");
            }
        }
        let token = self.make_next_token();
        // debug the token
        debug!("scanner next token: {:?}", token);
        Some(token)
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
        let token = scanner.next_token().unwrap();
        assert_eq!(token.get_number(), 1234);
    }

    #[test]
    fn test_sacnner_parse_keyword() {
        let mut builder = Builder::from_default_env();

        builder.filter(None, LevelFilter::Info).try_init();
        use super::*;
        let mut scanner =
            TinyCScanner::new(DoubleBufferStringStream::new_with_string("+".to_string()));
        let token = scanner.next_token().unwrap();
        assert_eq!(token.get_keyword(), "+");
    }

    #[test]
    fn test_scanner_parse_number_and_keyword() {
        let mut builder = Builder::from_default_env();

        builder.filter(None, LevelFilter::Info).try_init();
        use super::*;
        let mut scanner = TinyCScanner::new(DoubleBufferStringStream::new_with_string(
            "1234 +".to_string(),
        ));
        let token = scanner.next_token().unwrap();
        assert_eq!(token.get_number(), 1234);
        let token = scanner.next_token().unwrap();
        assert_eq!(token.get_keyword(), "+");
    }

    #[test]
    fn test_scanner_parse_expr() {
        let mut builder = Builder::from_default_env();

        builder.filter(None, LevelFilter::Info).try_init();

        use super::*;
        let mut scanner = TinyCScanner::new(DoubleBufferStringStream::new_with_string(
            "1234 + 1234".to_string(),
        ));
        let token = scanner.next_token().unwrap();
        assert_eq!(token.get_number(), 1234);
        let token = scanner.next_token().unwrap();
        assert_eq!(token.get_keyword(), "+");
        let token = scanner.next_token().unwrap();
        assert_eq!(token.get_number(), 1234);
    }
}
