use std::fmt;
use crate::lexer::Token;

/// Supported binary operators.
#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Eq,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Sub => write!(f, "-"),
            Operator::Mul => write!(f, "*"),
            Operator::Eq => write!(f, "=="),
        }
    }
}

/// AST for lambda expressions.
#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    Variable(String),
    Abstraction(String, Box<Term>),
    Application(Box<Term>, Box<Term>),
    Int(i32),
    Bool(bool),
    BinaryOp(Operator, Box<Term>, Box<Term>),
    IfElse(Box<Term>, Box<Term>, Box<Term>),
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Variable(s) => write!(f, "{}", s),
            Term::Abstraction(s, t) => write!(f, "(\\{}.{})", s, t),
            Term::Application(t1, t2) => write!(f, "({} {})", t1, t2),
            Term::Int(i) => write!(f, "{}", i),
            Term::Bool(b) => write!(f, "{}", b),
            Term::BinaryOp(op, t1, t2) => write!(f, "({} {} {})", t1, op, t2),
            Term::IfElse(cond, t1, t2) => write!(f, "if {} then {} else {}", cond, t1, t2),
        }
    }
}

pub fn parse(input: &str) -> Term {
    let tokens = crate::lexer::tokenize(input);
    let (term, remaining) = parse_term(&tokens);
    if !remaining.is_empty() {
        panic!("Unexpected input at end of string: {:?}", remaining);
    }
    term
}

fn parse_term(tokens: &[Token]) -> (Term, &[Token]) {
    match tokens.first() {
        Some(Token::LetTok) => {
            if let Some(Token::Literal(id)) = tokens.get(1) {
                if let Some(Token::EqTok) = tokens.get(2) {
                    let (t1, after_t1) = parse_term(&tokens[3..]);
                    match after_t1.first() {
                        Some(Token::InTok) => {
                            let (t2, after_t2) = parse_term(&after_t1[1..]);
                            let lambda = Term::Abstraction(id.clone(), Box::new(t2));
                            (Term::Application(Box::new(lambda), Box::new(t1)), after_t2)
                        },
                        _ => panic!("InTok expected"),
                    }
                } else { panic!("EqTok expected"); }
            } else { panic!("Literal expected after Let"); }
        }
        Some(Token::IfTok) => {
            let (cond, after_cond) = parse_term(&tokens[1..]);
            match after_cond.first() {
                Some(Token::ThenTok) => {
                    let (t_branch, after_t) = parse_term(&after_cond[1..]);
                    match after_t.first() {
                        Some(Token::ElseTok) => {
                            let (f_branch, after_f) = parse_term(&after_t[1..]);
                            (Term::IfElse(Box::new(cond), Box::new(t_branch), Box::new(f_branch)), after_f)
                        },
                        _ => panic!("ElseTok expected"),
                    }
                },
                _ => panic!("ThenTok expected"),
            }
        },
        Some(Token::LambdaTok) => {
            if let Some(Token::Literal(id)) = tokens.get(1) {
                if let Some(Token::DotTok) = tokens.get(2) {
                    let (body, after_body) = parse_term(&tokens[3..]);
                    (Term::Abstraction(id.clone(), Box::new(body)), after_body)
                } else { panic!("DotTok expected after variable in lambda"); }
            } else { panic!("Literal expected after LambdaTok"); }
        }
        _ => parse_eq(tokens)
    }
}

fn parse_eq(tokens: &[Token]) -> (Term, &[Token]) {
    let (mut left, mut rest) = parse_add(tokens);
    while let Some(Token::Equals) = rest.first() {
        let (right, next_rest) = parse_add(&rest[1..]);
        left = Term::BinaryOp(Operator::Eq, Box::new(left), Box::new(right));
        rest = next_rest;
    }
    (left, rest)
}

fn parse_add(tokens: &[Token]) -> (Term, &[Token]) {
    let (mut left, mut rest) = parse_mul(tokens);
    while let Some(tok) = rest.first() {
        let op = match tok {
            Token::Plus => Operator::Add,
            Token::Minus => Operator::Sub,
            _ => break,
        };
        let (right, next_rest) = parse_mul(&rest[1..]);
        left = Term::BinaryOp(op, Box::new(left), Box::new(right));
        rest = next_rest;
    }
    (left, rest)
}

fn parse_mul(tokens: &[Token]) -> (Term, &[Token]) {
    let (mut left, mut rest) = parse_app(tokens);
    while let Some(Token::Asterisk) = rest.first() {
        let (right, next_rest) = parse_app(&rest[1..]);
        left = Term::BinaryOp(Operator::Mul, Box::new(left), Box::new(right));
        rest = next_rest;
    }
    (left, rest)
}

fn parse_app(tokens: &[Token]) -> (Term, &[Token]) {
    let (mut left, mut rest) = parse_primary(tokens);
    
    while let Some(tok) = rest.first() {
        match tok {
            Token::Number(_) | Token::TrueTok | Token::FalseTok | Token::Literal(_) | Token::LParen => {
                let (right, next_rest) = parse_primary(rest);
                left = Term::Application(Box::new(left), Box::new(right));
                rest = next_rest;
            },
            _ => break,
        }
    }
    (left, rest)
}

fn parse_primary(tokens: &[Token]) -> (Term, &[Token]) {
    match tokens.first() {
        Some(Token::Number(n)) => (Term::Int(*n), &tokens[1..]),
        Some(Token::TrueTok) => (Term::Bool(true), &tokens[1..]),
        Some(Token::FalseTok) => (Term::Bool(false), &tokens[1..]),
        Some(Token::Literal(id)) => (Term::Variable(id.clone()), &tokens[1..]),
        Some(Token::LParen) => {
            let (term, rest) = parse_term(&tokens[1..]);
            match rest.first() {
                Some(Token::RParen) => (term, &rest[1..]),
                _ => panic!("RParen expected, got {:?}", rest.first()),
            }
        },
        _ => panic!("Unexpected token in primary: {:?}", tokens.first()),
    }
}
