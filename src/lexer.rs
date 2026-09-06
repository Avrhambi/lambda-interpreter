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
            '=' => { tokens.push(Token::EqTok); chars.next(); },
            ' ' | '\n' | '\r' | '\t' => { chars.next(); }, // Skip whitespace
            _ => {
                // Parse identifier
                let mut id = String::new();
                while let Some(&next_c) = chars.peek() {
                    if next_c.is_alphanumeric() || next_c == '\'' || next_c == '_' {
                         id.push(next_c);
                         chars.next();
                    } else {
                        break;
                    }
                }
                
                match id.as_str() {
                    "let" => tokens.push(Token::LetTok),
                    "in" => tokens.push(Token::InTok),
                    _ => tokens.push(Token::Literal(id)),
                }
            }
        }
    }
    tokens
}