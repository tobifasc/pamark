#[derive(Debug, PartialEq)]
pub enum LexItem {
    Char(char),
    Dash,
    Space,
    Hash,
    NewLine,
}
pub struct Lexer {
    pub chars: Vec<char>,
    pub current: usize,
    pub lex_items: Vec<LexItem>,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Lexer {
            chars: source.clone().chars().collect(),
            current: 0,
            lex_items: Vec::new(),
        }
    }
    fn add_item(&mut self, c: char) {
        match c {
            '#' => {
                self.lex_items.push(LexItem::Hash);
            }
            '\n' => {
                self.lex_items.push(LexItem::NewLine);
            }
            ' ' => {
                self.lex_items.push(LexItem::Space);
            }
            '-' => {
                self.lex_items.push(LexItem::Dash);
            }
            x => {
                self.lex_items.push(LexItem::Char(x));
            }
        }
    }
    pub fn lex(&mut self) {
        while let Some(c) = self.next() {
            self.add_item(c);
        }
    }
}
impl Iterator for Lexer {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.chars.len() {
            return None;
        }
        let result = self.chars[self.current];
        self.current += 1;
        Some(result)
    }
}
