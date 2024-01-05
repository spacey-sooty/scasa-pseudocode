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

#[cfg(test)]
mod tests {
    mod individual_chars {
        use crate::lexer::{lex_input, Token};

        #[test]
        fn test_plus() {
            let plus = lex_input("+".to_string());
            assert_eq!(plus.first().unwrap(), &Token::Plus);
            assert_eq!(plus.len(), 1);
        }

        #[test]
        fn test_minus() {
            let minus = lex_input("-".to_string());
            assert_eq!(minus.first().unwrap(), &Token::Minus);
            assert_eq!(minus.len(), 1);
        }

        #[test]
        fn test_star() {
            let star = lex_input("*".to_string());
            assert_eq!(star.first().unwrap(), &Token::Star);
            assert_eq!(star.len(), 1);
        }

        #[test]
        fn test_slash() {
            let slash = lex_input("/".to_string());
            assert_eq!(slash.first().unwrap(), &Token::Slash);
            assert_eq!(slash.len(), 1);
        }

        #[test]
        fn test_modulo() {
            let modulo = lex_input("%".to_string());
            assert_eq!(modulo.first().unwrap(), &Token::Modulo);
            assert_eq!(modulo.len(), 1);
        }

        #[test]
        fn test_rparen() {
            let rparen = lex_input(")".to_string());
            assert_eq!(rparen.first().unwrap(), &Token::RParen);
            assert_eq!(rparen.len(), 1);
        }

        #[test]
        fn test_lparen() {
            let lparen = lex_input("(".to_string());
            assert_eq!(lparen.first().unwrap(), &Token::LParen);
            assert_eq!(lparen.len(), 1);
        }

        #[test]
        fn test_nums() {
            let num = lex_input("0123456789".to_string());
            assert_eq!(num.first().unwrap(), &Token::Number(0123456789));
            assert_eq!(num.len(), 1);

            let nums = lex_input("0 1 2 3 4 5 6 7 8 9".to_string());
            let mut count = 0;
            for i in &nums {
                assert_eq!(i, &Token::Number(count));
                count += 1;
            }
            assert_eq!(nums.len(), 10);
        }

        #[test]
        fn test_whitespace() {
            let whitespace = lex_input(" \n\t\r".to_string());
            assert_eq!(whitespace.first(), None);
            assert_eq!(whitespace.len(), 0);
        }
    }
}
