use std::fmt;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Token {
    Lambda,
    Name(String),
    Dot,
    LBrace,
    RBrace
}
impl fmt::Display for Token {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Lambda => write!(f, "\\"),
            Token::Name(name) => f.write_str(name),
            Token::Dot => write!(f, "."),
            Token::LBrace => write!(f, "("),
            Token::RBrace => write!(f, ")")
        }
    }
}

#[derive(Debug, Clone)]
pub enum Term {
    Name(usize),
    Pair(Box<Term>, Box<Term>),
    Lambda(usize, Box<Term>)
}

impl Term {
    // pub fn is_name(&self) -> bool {
    //     if let Term::Name(_) = self { true } else { false }
    // }
    // pub fn is_pair(&self) -> bool {
    //     if let Term::Pair(_, _) = self { true } else { false }
    // }
    pub fn is_lambda(&self) -> bool {
        if let Term::Lambda(_, _) = self { true } else { false }
    }
}

pub type NameMap = HashMap<usize, String>;