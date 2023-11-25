mod lexer;
mod parser;

fn main() {
    let input = "1 + 2 * 3 - 4 / 5 % 6";
    let tokens = lexer::lex_input(input.to_string());
    println!("{:?}\n", tokens);

    let ast = parser::parse_lexed_input(tokens);
    ast.traverse();
}
