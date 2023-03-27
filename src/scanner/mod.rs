use crate::string_stream::{DoubleBufferStringStream, StreamBuffer, StringStream};
// use simplel logger to print log
use log::{error, info};
// Token has three types
// 1. keyword
// 2. number
// 3. eof (end of file)
#[derive(Debug,PartialEq, Eq)]
pub(crate) enum TokenType {
    Keyword,
    Number,
    Eof,
}

// Token is a pair of TokenType and the string
#[derive(Debug,PartialEq, Eq)]
pub(crate) struct Token {
    token_type: TokenType,
    string: String,
    number: Option<i32>,
}

impl Token {
    pub(crate) fn new() -> Token {
        Token {
            token_type: TokenType::Eof,
            string: String::new(),
            number: None,
        }
    }

    // get numbe r
    pub(crate) fn get_number(&self) -> i32 {
        assert!(!(self.token_type != TokenType::Number), "Token is not a number");
        self.number.unwrap()
    }
}

pub(crate) trait Scanner {
    fn next_token(&mut self) -> Option<Token>;
}

pub(crate) struct TinyCScanner {
    string_stream: DoubleBufferStringStream,
    current_buffer : Option<StreamBuffer>,
}

pub(crate) fn equal_token(token: &Token, token_type: TokenType, string: &str) -> bool {
    token.token_type == token_type && token.string == string
}

// return first number and the index of the first non-digit char
// if the string is empty, return (0,0)
pub(crate) fn strol(string: &[char],index : & mut usize) -> u64 {
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
        info!("buffer cnt : {} index {} ", buffer.count,buffer.read_index);
        let mut token = Token::new();
        let mut  index =  buffer.read_index;
        while index < buffer.count {
            let c = buffer.buffer[index];
            if c.is_ascii_digit() {
                // get the number
                let before_index = index;
                info!("current buf {}", &buffer.buffer[before_index..].iter().collect::<String>());
                let number = strol(&buffer.buffer, &mut index);
                // debug the number
                info!("parse from buffer: {} to {} str \"{}\" number {}", before_index, index, 
                &buffer.buffer[before_index..index].iter().collect::<String>(), number);
                token.token_type = TokenType::Number;
                token.number = Some(number as i32);
                break;
            } else if c.is_ascii_whitespace() {
                // skip whitespace
                index += 1;
                info!("skip whitespace");
            } else if c == '+' || c == '-' {
                info!("for preverse future, skip + or -");
                index += 1;
                token.token_type = TokenType::Keyword;
                token.string.push(c);
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

    // new 
    pub(crate) fn new(string_stream: DoubleBufferStringStream) -> TinyCScanner {
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
            }else {
                info!("get a new buffer");
            } 
        }
        let token = self.make_next_token();
        // debug the token 
        info!("token: {:?}", token);
        Some(token)
    }
}

// test 
#[cfg(test)]
mod tests{

    

    use env_logger::Builder;
    use log::LevelFilter;


    #[test]
    fn test_scanner_parse_number(){ 
        let mut builder = Builder::from_default_env();

        builder
        .filter(None, LevelFilter::Info)
        .try_init();
        use super::*;
        let mut scanner = TinyCScanner::new(DoubleBufferStringStream::new_with_string("1234".to_string()));
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Number);
        assert_eq!(token.number.unwrap(), 1234);
    }

    #[test]
    fn test_sacnner_parse_keyword() {
        let mut builder = Builder::from_default_env();

        builder
        .filter(None, LevelFilter::Info)
        .try_init();
        use super::*;
        let mut scanner = TinyCScanner::new(DoubleBufferStringStream::new_with_string("+".to_string()));
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Keyword);
        assert_eq!(token.string, "+");
    }

    #[test]
    fn test_scanner_parse_number_and_keyword(){
        let mut builder = Builder::from_default_env();

        builder
        .filter(None, LevelFilter::Info)
        .try_init();
        use super::*; 
        let mut scanner = TinyCScanner::new(DoubleBufferStringStream::new_with_string("1234 +".to_string()));
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Number);
        assert_eq!(token.number.unwrap(), 1234);
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Keyword);
        assert_eq!(token.string, "+");
    }

    #[test]
    fn test_scanner_parse_expr(){
        let mut builder = Builder::from_default_env();

        builder
        .filter(None, LevelFilter::Info)
        .try_init();

        use super::*;
        let mut scanner = TinyCScanner::new(DoubleBufferStringStream::new_with_string("1234 + 1234".to_string()));
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Number);
        assert_eq!(token.number.unwrap(), 1234);
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Keyword);
        assert_eq!(token.string, "+");
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Number);
        assert_eq!(token.number.unwrap(), 1234);
        
    }
}