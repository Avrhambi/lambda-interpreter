use std::collections::HashSet;
use crate::parser::Term;
use crate::utils;


fn fv(t: &Term) -> HashSet<String> {
    match t {
        Term::Variable(x) => {
            let mut set = HashSet::new();
            set.insert(x.clone());
            set
        },
        Term::Abstraction(x, body) => {
            let mut set = fv(body);
            set.remove(x);
            set
        },
        Term::Application(t1, t2) => {
            let s1 = fv(t1);
            let s2 = fv(t2);
            s1.union(&s2).cloned().collect()
        },
        _ => HashSet::new(),
    }
}


fn substitute(x: &str, t1: &Term, t2: &Term) -> Term {
    match t2 {
        Term::Variable(y) => {
            if y == x {
                t1.clone()
            } else {
                t2.clone()
            }
        },
        Term::Application(left, right) => {
            Term::Application(
                Box::new(substitute(x, t1, left)),
                Box::new(substitute(x, t1, right))
            )
        },
        Term::Abstraction(y, body) => {
            if y == x {
                t2.clone()
            } else if !fv(t1).contains(y) {
            
                Term::Abstraction(y.clone(), Box::new(substitute(x, t1, body)))
            } else { 
                let mut used = fv(t1);
                used.extend(fv(body));
                used.insert(x.to_string());
                let z = utils::fresh_var(&used);
                
                let new_body = substitute(y, &Term::Variable(z.clone()), body);
                Term::Abstraction(z, Box::new(substitute(x, t1, &new_body)))
            }
        },
        _ => t2.clone(),
    }
}


pub fn reduce_cbv(t: &Term) -> Option<Term> {
    match t {
        
        Term::Application(t1, t2) => {
            if let Term::Abstraction(x, body) = &**t1 {
                if let Term::Abstraction(_, _) = &**t2 {
                    return Some(substitute(x, t2, body));
                }
            }      
            if let Some(t1_prime) = reduce_cbv(t1) {
                return Some(Term::Application(Box::new(t1_prime), t2.clone()));
            }      
            if let Term::Abstraction(_, _) = &**t1 {
                if let Some(t2_prime) = reduce_cbv(t2) {
                    return Some(Term::Application(t1.clone(), Box::new(t2_prime)));
                }
            }
            None
        },
        _ => None, 
    }
}


pub fn reduce_cbn(t: &Term) -> Option<Term> {
    match t {
        Term::Application(t1, t2) => {
            if let Term::Abstraction(x, body) = &**t1 {
                return Some(substitute(x, t2, body));
            }
            
            if let Some(t1_prime) = reduce_cbn(t1) {
                return Some(Term::Application(Box::new(t1_prime), t2.clone()));
            }
            None
        },
        _ => None,
    }
}