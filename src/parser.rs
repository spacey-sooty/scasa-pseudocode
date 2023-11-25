use crate::lexer;

#[derive(Clone, Debug)]
pub struct Node {
    token: lexer::Token,
    child: Option<lexer::Token>,
}

#[derive(Debug)]
pub struct AST {
    pub root: Node,
    pub stored: Vec<Node>,
}

impl AST {
    pub fn traverse(&self) {
        for i in &self.stored {
            println!("{:?} -> ", i);
        }
    }
}

pub fn parse_lexed_input(tokens: Vec<lexer::Token>) -> AST {
    let mut iter_tokens = tokens.iter().peekable();
    let mut nodes: Vec<Node> = Vec::new();

    while let Some(token) = iter_tokens.next() {
        match token {
            lexer::Token::Number(num) => {
                nodes.push(Node {
                    token: lexer::Token::Number(*num),
                    child: iter_tokens.peek().cloned().cloned(),
                });
            }
            lexer::Token::Minus => {
                nodes.push(Node {
                    token: lexer::Token::Minus,
                    child: iter_tokens.peek().cloned().cloned(),
                });
            }
            lexer::Token::Plus => {
                nodes.push(Node {
                    token: lexer::Token::Plus,
                    child: iter_tokens.peek().cloned().cloned(),
                })
            }
            lexer::Token::Star => {
                nodes.push(Node {
                    token: lexer::Token::Star,
                    child: iter_tokens.peek().cloned().cloned(),
                });
            }
            lexer::Token::Slash => {
                nodes.push(Node {
                    token: lexer::Token::Slash,
                    child: iter_tokens.peek().cloned().cloned(),
                });
            }
            lexer::Token::Modulo => {
                nodes.push(Node {
                    token: lexer::Token::Modulo,
                    child: iter_tokens.next().cloned(),
                });
            }
            lexer::Token::LParen => {
                nodes.push(Node {
                    token: lexer::Token::LParen,
                    child: iter_tokens.peek().cloned().cloned(),
                });
            }
            lexer::Token::RParen => {
                nodes.push(Node {
                    token: lexer::Token::RParen,
                    child: iter_tokens.peek().cloned().cloned(),
                });
            }
        }
    }

    AST {
        root: nodes.first().unwrap().clone(),
        stored: nodes,
    }
}
