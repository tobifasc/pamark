mod lexer;
mod parser;
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
    println!("result: {:?}", parser.tokens);
}
