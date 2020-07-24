use super::types::*;

use std::fmt;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ParseError {
    message : String,
    idx : usize
}
impl fmt::Display for ParseError {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse error at index {}: {}", self.idx, self.message)
    }
}

pub type ParseResult = Result<(Term, NameMap), ParseError>;

struct Parser<'a> {
    tokens : &'a Vec<Token>,
    name2id : HashMap<String, Vec<usize>>,
    id2name : NameMap,
    used : usize,
    cursor : usize
}

impl<'a> Parser<'a> {
    fn new(tokens : &'a Vec<Token>) -> Parser<'a> {
        Parser{ tokens: tokens, name2id: HashMap::new(), id2name: HashMap::new(), used : 0, cursor : 0 }
    }

    fn parse_short(&mut self, is_braced : bool) -> Result<Term, ParseError> {
        if self.cursor >= self.tokens.len() {
            return Err(ParseError{ message: "the given lambda is incomplete.".to_string(), idx : self.cursor });
        }

        let cur = self.cursor;
        self.cursor += 1;
        match &self.tokens[cur] {
            Token::Name(ref name) => {
                if let Some(id_stack) = self.name2id.get(name) {
                    Ok(Term::Name(*id_stack.last().unwrap()))
                }
                else {
                    self.used += 1;
                    self.name2id.insert(name.clone(), vec![self.used; 1]);
                    self.id2name.insert(self.used, name.clone());
                    Ok(Term::Name(self.used))
                }
            },
            Token::LBrace => {
                match self.parse_long(true) {
                    Ok(t) => {
                        if self.cursor >= self.tokens.len() {
                            Err(ParseError{ message: "the given lambda is incomplete.".to_string(), idx : self.cursor })
                        }
                        else if let Token::RBrace = self.tokens[self.cursor] {
                            self.cursor += 1;
                            Ok(t)
                        }
                        else {
                            Err(ParseError{ message: "the given lambda is incomplete.".to_string(), idx : self.cursor })
                        }
                    }
                    Err(e) => Err(e)
                }
            },
            Token::Lambda => {
                if self.cursor + 2 >= self.tokens.len() {
                    Err(ParseError{ message: "the given lambda is incomplete.".to_string(), idx : self.cursor })
                }
                else {
                    let prev_cursor = self.cursor;
                    if let Token::Name(ref name) = self.tokens[self.cursor] {
                        if let Token::Dot = self.tokens[self.cursor + 1] {
                            self.used += 1;
                            if let Some(id_stack) = self.name2id.get_mut(name) {
                                id_stack.push(self.used);
                            }
                            else {
                                self.name2id.insert(name.clone(), vec![self.used; 1]);
                            }
                            self.id2name.insert(self.used, name.clone());

                            let ret = match self.parse_short(is_braced) {
                                Ok(t) => {
                                    if let Term::Name(id) = t {
                                        self.cursor += 1;
                                        match self.parse_long(is_braced) {
                                            Ok(t) => {
                                                Ok(Term::Lambda(id, Box::new(t)))
                                            },
                                            Err(e) => Err(e)
                                        }
                                    }
                                    else {
                                        unreachable!();
                                    }
                                }
                                Err(e) => Err(e)
                            };
                            
                            if ret.is_ok() {
                                let id_stack = self.name2id.get_mut(name).unwrap();
                                id_stack.pop();
                                if id_stack.is_empty() {
                                    self.name2id.remove(name);
                                }
                            }
                            ret
                        }
                        else {
                            Err(ParseError{ message: "expect '.' but another token was given.".to_string(), idx : prev_cursor + 1 })
                        }
                    }
                    else {
                        Err(ParseError{ message: "'\\' must be followed by a Name.".to_string(), idx : prev_cursor })
                    }
                }
            },
            tok => Err(ParseError{ message: format!("unexpected token '{}'.", &tok), idx : cur})
        }
    }
    fn parse_long(&mut self, is_braced : bool) -> Result<Term, ParseError> {
        if self.cursor >= self.tokens.len() {
            return Err(ParseError{ message: "the given lambda is incomplete.".to_string(), idx : self.cursor });
        }
        
        let term = self.parse_short(is_braced);
        if term.is_err() {
            return term;
        }
        let mut term = term.ok().unwrap();
    
        while self.cursor < self.tokens.len() {
            match self.tokens[self.cursor] {
                Token::RBrace => {
                    if is_braced {
                        return Ok(term);
                    }
                    else {
                        return Err(ParseError{ message: "unexpected token ')'.".to_string(), idx : self.cursor });
                    }
                },
                _ => {
                    match self.parse_short(is_braced) {
                        Ok(t) => {
                            term = Term::Pair(Box::new(term), Box::new(t));
                        },
                        Err(e) => { return Err(e); }
                    }
                }
            }
        }
    
        Ok(term)
    }
}

pub fn parse(tokens : &Vec<Token>) -> ParseResult {
    let mut parser = Parser::new(tokens);
    match parser.parse_long(false) {
        Ok(term) => Ok((term, parser.id2name)),
        Err(e) => Err(e)
    }
}