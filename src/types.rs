use std::fmt;

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