mod lexer;
mod parser;
mod renderer;

fn main() {

    let input = std::fs::read_to_string("input.md").expect("Unable to read file");

    let mut lexer = lexer::Lexer::new(input);
    lexer.lex();

    let mut parser = parser::Parser::new(lexer.lex_items);
    parser.parse();

    let mut renderer = renderer::Renderer::new(parser.tokens);
    let result = renderer.render();

    std::fs::write("./result.html", result).expect("Unable to write file");
}
