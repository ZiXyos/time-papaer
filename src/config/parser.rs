use crate::config::types::Config;

use super::types::{Mode, Lexer, Token};

impl Config {

    pub fn get_mode(&self) -> &Mode {
    
       return &self.mode;
    }

    fn set_mode(&mut self, mode: Mode) {

        self.mode = mode;
    }
}

impl<'lt> Lexer<'lt> {
    pub fn new(input: &'lt str) -> Lexer<'lt> {

        Lexer {
            input,
            pos: 0
        }
    }

    fn next_token(&mut self) -> Token {
        if self.pos >= self.input.len() {
            return Token::EOF;
        }

        let c = self.input.chars().nth(self.pos).unwrap();
        self.pos += 1;

        match c {
            '=' => Token::Equal,
            'a'..='z' => {
                    let start = self.pos - 1;
                    while self.pos < self.input.len() {
                        
                        let next_char = self.input.chars().nth(self.pos).unwrap();
                        if !next_char.is_ascii() {
                            break;
                        }
                        self.pos += 1;
                    }
                    let keyword = &self.input[start..self.pos];
                    Token::KeyWord(keyword.to_string())
                }
            _ => panic!("Invalid character")
        }
    }
}
