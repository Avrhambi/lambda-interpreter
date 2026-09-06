#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    LambdaTok,
    EqTok,
    DotTok,
    LetTok,
    InTok,
    Literal(String),
    LParen,
    RParen,
    Number(i32),
    TrueTok,
    FalseTok,
    Plus,
    Minus,
    Asterisk,
    Equals,
    IfTok,
    ThenTok,
    ElseTok,
}

/// Tokenizes the input string into a vector of Tokens.
pub fn tokenize(input: &str) -> Vec<Token> {
    let mut chars = input.chars().peekable();
    let mut tokens = Vec::new();

    while let Some(&c) = chars.peek() {
        match c {
            '\\' => { tokens.push(Token::LambdaTok); chars.next(); },
            '.' => { tokens.push(Token::DotTok); chars.next(); },
            '(' => { tokens.push(Token::LParen); chars.next(); },
            ')' => { tokens.push(Token::RParen); chars.next(); },
            '+' => { tokens.push(Token::Plus); chars.next(); },
            '-' => { tokens.push(Token::Minus); chars.next(); },
            '*' => { tokens.push(Token::Asterisk); chars.next(); },
            '=' => {
                chars.next();
                if let Some(&'=') = chars.peek() {
                    tokens.push(Token::Equals);
                    chars.next();
                } else {
                    tokens.push(Token::EqTok);
                }
            },
            ' ' | '\n' | '\r' | '\t' => { chars.next(); }, // Skip whitespace
            c if c.is_ascii_digit() => {
                let mut num = String::new();
                while let Some(&next_c) = chars.peek() {
                    if next_c.is_ascii_digit() {
                        num.push(next_c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(num.parse().unwrap()));
            },
            _ => {
                // Parse identifier
                let mut id = String::new();
                while let Some(&next_c) = chars.peek() {
                    if next_c.is_alphabetic() || next_c.is_ascii_digit() || next_c == '\'' || next_c == '_' {
                         id.push(next_c);
                         chars.next();
                    } else {
                        break;
                    }
                }
                
                if id.is_empty() {
                    panic!("Unexpected character: {}", chars.next().unwrap());
                }
                
                match id.as_str() {
                    "let" => tokens.push(Token::LetTok),
                    "in" => tokens.push(Token::InTok),
                    "true" => tokens.push(Token::TrueTok),
                    "false" => tokens.push(Token::FalseTok),
                    "if" => tokens.push(Token::IfTok),
                    "then" => tokens.push(Token::ThenTok),
                    "else" => tokens.push(Token::ElseTok),
                    _ => tokens.push(Token::Literal(id)),
                }
            }
        }
    }
    tokens
}