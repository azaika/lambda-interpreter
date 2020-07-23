use super::types::*;
use super::lexer;

use std::fmt;
use std::collections::HashMap;

pub struct ParseError {
    message : String,
    idx : usize
}
impl fmt::Display for ParseError {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse error at index {}: {}", self.idx, self.message)
    }
}
