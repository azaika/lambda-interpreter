use std::rc::Rc;

#[derive(Clone)]
pub struct Name {
    id : usize,
    name : String
}

#[derive(Clone)]
pub enum Term {
    Variable(Name),
    Pair(Box<Term>, Box<Term>),
    Lambda(Name, Box<Term>)
}