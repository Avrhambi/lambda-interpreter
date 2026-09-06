use std::fmt;
use crate::lexer::Token;

/// AST for lambda expressions.
/// Using Box<Term> because Rust requires recursive types to have a known size.
#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    Variable(String),
    Abstraction(String, Box<Term>),
    Application(Box<Term>, Box<Term>),
}

/// Implementation of Display for pretty printing the terms.
impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Variable(s) => write!(f, "{}", s),
            Term::Abstraction(s, t) => write!(f, "(\\{}.{})", s, t),
            Term::Application(t1, t2) => write!(f, "({} {})", t1, t2),
        }
    }
}

/// Main parse function exposed to other modules.
pub fn parse(input: &str) -> Term {
    let tokens = crate::lexer::tokenize(input);
    let (term, remaining) = parse_term(&tokens);
    if !remaining.is_empty() {
        panic!("Unexpected input at end of string");
    }
    term
}

/// Recursive helper function to parse terms from a slice of tokens.
fn parse_term(tokens: &[Token]) -> (Term, &[Token]) {
    match tokens.first() {
        None => panic!("Tokens expected"),
        Some(Token::Literal(id)) => (Term::Variable(id.clone()), &tokens[1..]),
        Some(Token::LParen) => {
            let rest = &tokens[1..];
            // Lookahead: Check if it's a Lambda: (\x.t)
            if let Some(Token::LambdaTok) = rest.first() {
                if let Some(Token::Literal(id)) = rest.get(1) {
                    if let Some(Token::DotTok) = rest.get(2) {
                        let (body, after_body) = parse_term(&rest[3..]);
                        match after_body.first() {
                            Some(Token::RParen) => {
                                return (Term::Abstraction(id.clone(), Box::new(body)), &after_body[1..]);
                            },
                            _ => panic!("RParen expected after lambda body"),
                        }
                    }
                }
            }
            
            // Standard application or parenthesized term
            let (t1, after_t1) = parse_term(rest);
            match after_t1.first() {
                Some(Token::RParen) => (t1, &after_t1[1..]), // Just parens (t)
                _ => {
                    let (t2, after_t2) = parse_term(after_t1);
                    match after_t2.first() {
                        Some(Token::RParen) => {
                            (Term::Application(Box::new(t1), Box::new(t2)), &after_t2[1..])
                        },
                        _ => panic!("RParen expected after application"),
                    }
                }
            }
        },
        Some(Token::LetTok) => {
            // Sugar: let id = t1 in t2  ==>  ((\id.t2) t1)
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
        _ => panic!("Unexpected token"),
    }
}