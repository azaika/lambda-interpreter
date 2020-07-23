use super::types::*;

use std::fmt;

#[derive(Debug)]
pub struct LexerError {
    idx : usize,
    invalid_char : char
}
impl fmt::Display for LexerError {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid char {} at letter {}.", self.invalid_char, self.idx)
    }
}

pub type LexerResult = Result<Vec<Token>, LexerError>;

pub fn tokenize(source : &str) -> LexerResult {
    let mut tokens : Vec<Token> = Vec::new();

    let mut buf : Option<String> = None;
    for (idx, c) in source.chars().enumerate() {
        if buf.is_some() {
            if c.is_ascii_alphabetic() {
                buf.as_mut().unwrap().push(c);
                continue;
            }
            
            tokens.push(Token::Name(buf.unwrap()));
            buf = None;
        }
        
        if c.is_ascii_alphabetic() {
            buf = Some(c.to_string());
        }
        else if c == '\n' || c == ' ' {
            continue;
        }
        else if c == '\\' {
            tokens.push(Token::Lambda);
        }
        else if c == '.' {
            tokens.push(Token::Dot);
        }
        else if c == '(' {
            tokens.push(Token::LBrace);
        }
        else if c == ')' {
            tokens.push(Token::RBrace);
        }
        else {
            return Err(LexerError{idx : idx, invalid_char : c});
        }
    }

    if let Some(name) = buf {
        tokens.push(Token::Name(name));
    }

    Ok(tokens)
}