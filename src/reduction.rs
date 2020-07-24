use super::types::*;

use std::fmt;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct Reducer {
    id2name : NameMap,
    used : usize
}

impl Reducer {
    pub fn new(id2name : NameMap) -> Reducer {
        let used = *id2name.iter().map(|(k, _)|{ k }).max().unwrap_or(&0);
        Reducer{ id2name: id2name, used: used }
    }

    fn find_free_var(&self, term : &Term, ids : &mut HashSet<String>) {
        match term {
            Term::Name(id) => {
                ids.insert(self.id2name.get(id).unwrap().clone());
            },
            Term::Pair(t1, t2) => {
                self.find_free_var(t1, ids);
                self.find_free_var(t2, ids);
            }
            Term::Lambda(id, inner) => {
                let name = self.id2name.get(id).unwrap();
                ids.insert(name.clone());
                self.find_free_var(inner, ids);
                ids.remove(name);
            }
        }
    }

    fn replace_id(&mut self, term : Term, table : &mut HashMap<usize, usize>) -> Term {
        match term {
            Term::Name(id) => {
                Term::Name(*table.get(&id).unwrap_or(&id))
            },
            Term::Pair(l, r) => {
                Term::Pair(Box::new(self.replace_id(*l, table)), Box::new(self.replace_id(*r, table)))
            },
            Term::Lambda(arg, inner) => {
                self.used += 1;
                let new_id = self.used;
                table.insert(arg, new_id);
                self.id2name.insert(new_id, self.id2name.get(&arg).unwrap().clone());
                let new_inner = Box::new(self.replace_id(*inner, table));
                table.remove(&arg);

                Term::Lambda(new_id, new_inner)
            }
        }
    }

    fn assign(&mut self, id : usize, t1: Term, t2 : &Term, free_vars : &HashSet<String>) -> Term {
        match t1 {
            Term::Name(id_) => {
                if id_ == id {
                    if t2.is_lambda() {
                        let mut table : HashMap<usize, usize> = HashMap::new();
                        self.replace_id(t2.clone(), &mut table)
                    }
                    else {
                        t2.clone()
                    }
                }
                else {
                    t1
                }
            },
            Term::Pair(l, r) => {
                Term::Pair(Box::new(self.assign(id, *l, t2, free_vars)), Box::new(self.assign(id, *r, t2, free_vars)))
            },
            Term::Lambda(arg, inner) => {
                let name = self.id2name.get_mut(&arg).unwrap();

                while free_vars.contains(name) {
                    *name += "\'";
                }

                Term::Lambda(arg, Box::new(self.assign(id, *inner, t2, free_vars)))
            }
        }
    }

    pub fn reduce(&mut self, term : Term) -> (Term, bool) {
        match term {
            Term::Name(_) => { return (term, false); }
            Term::Pair(t1, t2) => {
                let (t1, is_reduced) = self.reduce(*t1);
                if is_reduced {
                    return (Term::Pair(Box::new(t1), t2), true);
                }
    
                let (t2, is_reduced) = self.reduce(*t2);
                if is_reduced {
                    return (Term::Pair(Box::new(t1), Box::new(t2)), true);
                }
    
                if let Term::Lambda(id, inner) = t1 {
                    // find all free variable in t2
                    let mut free_vars : HashSet<String> = HashSet::new();
                    self.find_free_var(&t2, &mut free_vars);

                    return (self.assign(id, *inner, &t2, &free_vars), true);
                }
                else {
                    return (Term::Pair(Box::new(t1), Box::new(t2)), false);
                }
            },
            Term::Lambda(id, inner) => {
                let (inner, is_reduced) = self.reduce(*inner);
                return (Term::Lambda(id, Box::new(inner)), is_reduced);
            }
        }
    }

    fn format(&self, term : &Term, f : &mut fmt::Formatter) -> fmt::Result {
        match term {
            Term::Name(id) => {
                write!(f, "{}", self.id2name.get(id).unwrap())
            },
            Term::Pair(t1, t2) => {
                let _ = write!(f, "(");
                let _ = self.format(t1.as_ref(), f);
                let _ = write!(f, " ");
                let _ = self.format(t2.as_ref(), f);
                write!(f, ")")
            },
            Term::Lambda(id, inner) => {
                let _ = write!(f, "(λ{}.", self.id2name.get(id).unwrap());
                let _ = self.format(inner.as_ref(), f);
                write!(f, ")")
            }
        }
    }
}

pub struct Formatter<'a, 'b> {
    red : &'a Reducer,
    term : &'b Term,
}
impl<'a, 'b> Formatter<'a, 'b> {
    pub fn new(red : &'a Reducer, term : &'b Term) -> Self {
        Self{ red:red, term:term }
    }
}
impl<'a, 'b> fmt::Display for Formatter<'a, 'b> {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        self.red.format(self.term, f)
    }
}