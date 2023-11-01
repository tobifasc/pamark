use crate::lexer::LexItem;
#[derive(Debug)]
pub enum Token {
    H1(String),
    H2(String),
    H3(String),
    Text(String),
    List(Vec<String>),
}

pub struct Parser {
    pub lex_items: Vec<LexItem>,
    pub current: usize,
    pub tokens: Vec<Token>,
}

impl Parser {
    pub fn new(lex_items: Vec<LexItem>) -> Parser {
        Parser {
            lex_items,
            current: 0,
            tokens: Vec::new(),
        }
    }
    fn parse_list(&mut self) -> Vec<String> {
        let mut result = Vec::new();
        let mut word = String::from("");
        while self.current < self.lex_items.len() {
            match &self.lex_items[self.current] {
                LexItem::NewLine => {
                    if self.is_prev_a(LexItem::NewLine) {
                        break;
                    }
                }
                LexItem::Space => {
                    if word.len() != 0 {
                        word.push(' ');
                    }
                }
                LexItem::Hash => {
                    word.push('#');
                }
                LexItem::Dash => {
                    // this will not parse dash in text correctly... ("hello - world")
                    if word.len() != 0 && !self.is_prev_a(LexItem::NewLine) && !self.is_prev_a(LexItem::Space)  {
                        word.push('-');
                    } else if word.len() != 0 {
                        result.push(word);
                        word = String::from("");
                    }
                }
                LexItem::Char(x) => {
                    word.push(*x);
                }
            }
            self.current += 1;
        }

        // handle file end
        if word.len() != 0 {
            result.push(word);
        }

        result
    }

    fn parse_heading(&mut self) -> Token {
        let mut result = String::from("");
        let mut heading_count = 0;
        while self.current < self.lex_items.len() {
            match &self.lex_items[self.current] {
                LexItem::NewLine => {
                    break;
                }
                LexItem::Space => {
                    if result.len() != 0 {
                        result.push(' ');
                    }
                }
                LexItem::Hash => {
                    if result.len() != 0 {
                        result.push('#');
                    } else {
                        heading_count += 1;
                    }
                }
                LexItem::Dash => {
                    result.push('-');
                }
                LexItem::Char(x) => {
                    result.push(*x);
                }
            }
            self.current += 1;
        }

        match heading_count {
            1 => {
                return Token::H1(result);
            }
            2 => {
                return Token::H2(result);
            }
            _ => {
                return Token::H3(result);
            }
        }
    }
    fn parse_paragraph(&mut self) -> String {
        let mut result = String::from("");
        while self.current < self.lex_items.len() {
            match &self.lex_items[self.current] {
                LexItem::NewLine => {
                    if self.is_prev_a(LexItem::NewLine) {
                        break;
                    }
                    result.push('\n');
                }
                LexItem::Space => {
                    result.push(' ');
                }
                LexItem::Hash => {
                    if self.is_prev_a(LexItem::NewLine) {
                        self.current = self.current - 1;
                        break;
                    }
                    result.push('#');
                }
                LexItem::Dash => {
                    if self.is_prev_a(LexItem::NewLine) {
                        self.current = self.current - 1;
                        break;
                    }
                    result.push('-');
                }
                LexItem::Char(x) => {
                    result.push(*x);
                }
            }
            self.current += 1;
        }
        return result;
    }

    fn is_prev_a(&self, compare: LexItem) -> bool {
        return self.lex_items[self.current - 1] == compare;
    }

    pub fn parse(&mut self) {
        while self.current < self.lex_items.len() {
            match &self.lex_items[self.current] {
                LexItem::Hash => {
                    let heading = self.parse_heading();
                    self.tokens.push(heading);
                }
                LexItem::NewLine => {}
                LexItem::Dash => {
                    let list = self.parse_list();
                    self.tokens.push(Token::List(list));
                }
                LexItem::Space => {
                    // ignore leading space
                }
                _ => {
                    let text = self.parse_paragraph();

                    self.tokens.push(Token::Text(text));
                }
            }
            self.current += 1;
        }
    }
}
