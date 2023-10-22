use crate::parser::Token;
/*pub enum Token {
    H1(String),
    H2(String),
    H3(String),
    Text(String),
    List(Vec<String>),
}*/

pub struct Renderer {
    pub tokens: Vec<Token>,
}

impl Renderer {
    pub fn new(tokens: Vec<Token>) -> Renderer {
        Renderer { tokens }
    }

    pub fn render(&mut self) -> String {
        let mut out = String::from("");
        println!("rendering...");
        for token in &self.tokens {
            match token {
                Token::H1(title) => {
                    out.push_str("<h1>");
                    out.push_str(&title);
                    out.push_str("</h1>\n");
                }
                Token::H2(title) => {
                    out.push_str("<h2>");
                    out.push_str(&title);
                    out.push_str("</h2>\n");
                }
                Token::H3(title) => {
                    out.push_str("<h3>");
                    out.push_str(&title);
                    out.push_str("</h3>\n");
                }
                Token::Text(text) => {
                    out.push_str("<p>");
                    out.push_str(&text);
                    out.push_str("</p>\n");
                }
                Token::List(list_items) => {
                    out.push_str("<ul>\n");
                    for list_item in list_items {
                        out.push_str("<li>");
                        out.push_str(list_item);
                        out.push_str("</li>\n");
                    }
                    out.push_str("</ul>\n");
                }
            }
        }
        out
    }
}
