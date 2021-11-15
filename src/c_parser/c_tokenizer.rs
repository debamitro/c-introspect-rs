use std::fs::File;
use std::io::{BufRead, BufReader};

use super::c_tokens::{Identifier, Token};

pub struct TokenItr<'a> {
    pos: usize,
    line: String,
    buf_reader: &'a mut BufReader<File>,
}

impl TokenItr<'_> {
    pub fn new(buf_reader: &mut BufReader<File>) -> TokenItr {
        TokenItr {
            pos: 0,
            line: String::new(),
            buf_reader,
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

    fn skip_whitespace(&mut self) -> Option<usize> {
        loop {
            for one_byte in (&self.line[self.pos..]).chars() {
                if one_byte.is_whitespace() {
                    self.pos += 1;
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
        return Some(self.pos);
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

        if let None = self.skip_whitespace() {
            return None;
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

                if &self.line[start..end] == "struct" {
                    Some(Token::STRUCT)
                } else {
                    Some(Token::IDENTIFIER(Identifier {
                        value: (&self.line[start..end]).to_string(),
                    }))
                }
            }
        }
    }
}
