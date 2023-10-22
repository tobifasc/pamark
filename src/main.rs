mod lexer;
mod parser;
mod renderer;
fn main() {
    let mut lexer = lexer::Lexer::new(String::from(
        "#  abcde
lol
was geht


ok neuer
text
### un djetzt
- i am 
- a 
- list

#hello",
    ));
    lexer.lex();
    let mut parser = parser::Parser::new(lexer.lex_items);
    parser.parse();
    let mut renderer = renderer::Renderer::new(parser.tokens);
    let result = renderer.render();
    //println!("result: {:?}", result);
    std::fs::write("./result.html", result).expect("Unable to write file");
}
