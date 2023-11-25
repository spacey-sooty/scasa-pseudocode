#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    Modulo,
    Number(i32),
}

pub fn lex_input(input: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            ' ' => continue,
            '\n' => continue,
            '\t' => continue,
            '\r' => continue,
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Star),
            '/' => tokens.push(Token::Slash),
            '%' => tokens.push(Token::Modulo),
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            '0'..='9' => {
                let mut num = c.to_string();
                while let Some('0'..='9') = chars.peek() {
                    num.push(chars.next().unwrap());
                }
                tokens.push(Token::Number(num.parse().unwrap()));
            }
            _ => panic!("Invalid Character: {}", c),
        }
    }
    tokens
}
