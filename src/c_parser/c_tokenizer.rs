use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::Vec;

use super::c_tokens::{Identifier, Token};

pub struct TokenItr<'a> {
    pos: usize,
    line: String,
    buf_reader: &'a mut BufReader<File>,
    pushed_back_tokens: Vec<Token>,
}

impl TokenItr<'_> {
    pub fn new(buf_reader: &mut BufReader<File>) -> TokenItr {
        TokenItr {
            pos: 0,
            line: String::new(),
            buf_reader,
            pushed_back_tokens: Vec::new(),
        }
    }

    fn read_next_line(&mut self) -> bool {
        self.pos = 0;
        loop {
            self.line.clear();
            match self.buf_reader.read_line(&mut self.line) {
                Ok(0) => {
                    return false;
                }
                Ok(c) => {
                    if c > 0 {
                        break;
                    }
                }
                Err(_) => {
                    return false;
                }
            }
        }

        return true;
    }

    fn skip_whitespace(&mut self) -> Option<bool> {
        let mut did_skip = false;
        loop {
            for one_byte in (&self.line[self.pos..]).chars() {
                if one_byte.is_whitespace() {
                    self.pos += 1;
                    did_skip = true;
                } else {
                    break;
                }
            }

            if self.pos >= self.line.len() {
                if self.read_next_line() == false {
                    return None;
                }
            } else {
                break;
            }
        }
        return Some(did_skip);
    }

    fn skip_comment(&mut self) -> Option<bool> {
        if self.line.len() - self.pos < 2 {
            // We need at least two characters to start a comment
            return Some(false);
        }

        if self.line[self.pos..].starts_with("//") {
            if self.read_next_line() == false {
                return None;
            } else {
                return Some(true);
            }
        }

        if self.line[self.pos..].starts_with("/*") {
            self.pos += 2;
            let mut previous_char: char = '*';
            loop {
                for one_char in (&self.line[self.pos..]).chars() {
                    self.pos += one_char.len_utf8();
                    if one_char == '/' && previous_char == '*' {
                        break;
                    } else {
                        previous_char = one_char;
                    }
                }

                if self.pos >= self.line.len() {
                    if self.read_next_line() == false {
                        return None;
                    }
                } else {
                    break;
                }
            }
            return Some(true);
        }

        return Some(false);
    }

    pub fn push_back(&mut self, tok: Token) {
        self.pushed_back_tokens.push(tok)
    }
}

impl Iterator for TokenItr<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.line.len() {
            if self.read_next_line() == false {
                return None;
            }
        }

        if let Some(tok) = self.pushed_back_tokens.pop() {
            return Some(tok);
        }

        loop {
            let mut did_skip: bool;

            match self.skip_whitespace() {
                Some(tf) => did_skip = tf,
                None => return None,
            }

            match self.skip_comment() {
                Some(tf) => did_skip = tf,
                None => return None,
            }

            if !did_skip {
                break;
            }
        }

        let mut itr = (&self.line[self.pos..]).chars();

        let start = self.pos;
        match itr.next() {
            Some('{') => {
                self.pos += 1;
                Some(Token::LBRACE)
            }
            Some('}') => {
                self.pos += 1;
                Some(Token::RBRACE)
            }
            Some(';') => {
                self.pos += 1;
                Some(Token::SEMICOLON)
            }
            _ => {
                let mut end: usize = start + 1;
                loop {
                    let one_byte = itr.next();
                    match one_byte {
                        Some(c) => {
                            if c.is_alphanumeric() || c == '_' {
                                end += 1;
                            } else {
                                break;
                            }
                        }
                        None => {
                            break;
                        }
                    }
                }
                self.pos = end;

                let token_text: &str = &self.line[start..end];
                match token_text {
                    "struct" => Some(Token::STRUCT),
                    "typedef" => Some(Token::TYPEDEF),
                    "class" => Some(Token::CLASS),
                    "int" => Some(Token::INT),
                    "long" => Some(Token::LONG),
                    _ => Some(Token::IDENTIFIER(Identifier {
                        value: token_text.to_string(),
                    })),
                }
            }
        }
    }
}
