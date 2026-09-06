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
        Term::BinaryOp(_, t1, t2) => {
            let s1 = fv(t1);
            let s2 = fv(t2);
            s1.union(&s2).cloned().collect()
        },
        Term::IfElse(cond, t1, t2) => {
            let mut s = fv(cond);
            s.extend(fv(t1));
            s.extend(fv(t2));
            s
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
        Term::BinaryOp(op, left, right) => {
            Term::BinaryOp(
                op.clone(),
                Box::new(substitute(x, t1, left)),
                Box::new(substitute(x, t1, right))
            )
        },
        Term::IfElse(cond, t_branch, f_branch) => {
            Term::IfElse(
                Box::new(substitute(x, t1, cond)),
                Box::new(substitute(x, t1, t_branch)),
                Box::new(substitute(x, t1, f_branch))
            )
        },
        _ => t2.clone(),
    }
}

fn is_value(t: &Term) -> bool {
    match t {
        Term::Abstraction(_, _) | Term::Int(_) | Term::Bool(_) => true,
        _ => false,
    }
}


pub fn reduce_cbv(t: &Term) -> Option<Term> {
    match t {
        
        Term::Application(t1, t2) => {
            if let Term::Abstraction(x, body) = &**t1 {
                if is_value(t2) {
                    return Some(substitute(x, t2, body));
                }
            }      
            if let Some(t1_prime) = reduce_cbv(t1) {
                return Some(Term::Application(Box::new(t1_prime), t2.clone()));
            }      
            if is_value(t1) {
                if let Some(t2_prime) = reduce_cbv(t2) {
                    return Some(Term::Application(t1.clone(), Box::new(t2_prime)));
                }
            }
            None
        },
        Term::BinaryOp(op, t1, t2) => {
            if let Some(t1_prime) = reduce_cbv(t1) {
                return Some(Term::BinaryOp(op.clone(), Box::new(t1_prime), t2.clone()));
            }
            if let Some(t2_prime) = reduce_cbv(t2) {
                return Some(Term::BinaryOp(op.clone(), t1.clone(), Box::new(t2_prime)));
            }
            if let (Term::Int(n1), Term::Int(n2)) = (&**t1, &**t2) {
                match op {
                    crate::parser::Operator::Add => Some(Term::Int(n1 + n2)),
                    crate::parser::Operator::Sub => Some(Term::Int(n1 - n2)),
                    crate::parser::Operator::Mul => Some(Term::Int(n1 * n2)),
                    crate::parser::Operator::Eq => Some(Term::Bool(n1 == n2)),
                }
            } else {
                None
            }
        },
        Term::IfElse(cond, t1, t2) => {
            if let Some(cond_prime) = reduce_cbv(cond) {
                return Some(Term::IfElse(Box::new(cond_prime), t1.clone(), t2.clone()));
            }
            if let Term::Bool(b) = &**cond {
                if *b {
                    Some(*t1.clone())
                } else {
                    Some(*t2.clone())
                }
            } else {
                None
            }
        },
        _ => None, 
    }
}
