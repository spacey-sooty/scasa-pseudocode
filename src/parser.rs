use crate::lexer;

pub struct Node {
    token: lexer::Token,
}

pub struct AST {
    pub root: Node,
    pub stored: Vec<Node>
}

pub fn parse_lexed_input(tokens: Vec<lexer::Token>) -> AST {
    let mut iter_tokens = tokens.iter();
    let mut nodes: Vec<Node> = Vec::new();
    let mut result: AST;

    while let Some(token) = iter_tokens.next() {
        if tokens.first() == Some(token) {
            result.root = token;
        }

        match token {
            lexer::Token::Number(num) => {
                nodes.push(Node {
                    token: lexer::Token::Number(*num),
                });
            },
            lexer::Token::Minus => {},
            lexer::Token::Plus => {},
            lexer::Token::Star => {},
            lexer::Token::Slash => {},
            lexer::Token::Modulo => {},
            lexer::Token::LParen => {},
            lexer::Token::RParen => {},
        }
    }
    result
}
