//! This module provides a routine called parse_c_file
//! which provides an iterator over all C structures in a file

use std::fs::File;
use std::io::BufReader;

mod c_tokenizer;
mod c_tokens;

use crate::c_structures::*;
use c_tokenizer::TokenItr;
use c_tokens::{token_value, Token};

fn parse_declaration(mut itr: TokenItr) -> Option<C_Declaration> {
    if let Some(tok1) = itr.next() {
        let valid_type = match tok1 {
            Token::INT => true,
            Token::LONG => true,
            Token::IDENTIFIER(_) => true,
            _ => false,
        };

        if !valid_type {
            return None;
        }

        if let Some(tok2) = itr.next() {
            if let Token::IDENTIFIER(_) = tok2 {
                return Some(C_Declaration {
                    typename: token_value(tok1),
                    name: token_value(tok2),
                });
            }
        }
    }

    return None;
}

fn parse_struct(mut itr: TokenItr) -> Option<C_Struct> {
    if let Some(tok1 @ Token::IDENTIFIER(_)) = itr.next() {
        if let Some(Token::LBRACE) = itr.next() {
            if let Some(d) = parse_declaration(itr) {
                return Some(C_Struct {
                    name: token_value(tok1),
                    fields: vec![d],
                });
            }
        }
    }

    return None;
}

pub struct C_Declaration {
    pub typename: String,
    pub name: String,
}

pub struct C_Struct {
    pub name: String,
    pub fields: Vec<C_Declaration>,
}

pub struct C_StructIter {
    finished: bool,
    buf_reader: BufReader<File>,
}

impl C_StructIter {
    fn new(f: File) -> C_StructIter {
        return C_StructIter {
            finished: false,
            buf_reader: BufReader::new(f),
        };
    }
}

impl Iterator for C_StructIter {
    type Item = C_Struct;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let mut itr = TokenItr::new(&mut self.buf_reader);

        match itr.next() {
            Some(t) => match t {
                Token::STRUCT => parse_struct(itr),
                _ => {
                    self.finished = true;
                    None
                }
            },
            _ => {
                self.finished = true;
                None
            }
        }
    }
}

/// This routine takes a file name
/// and returns an `Option` value of type
/// iterator
pub fn parse_c_file(filename: &str) -> Option<C_StructIter> {
    let f = File::open(filename);
    match f {
        Ok(of) => return Some(C_StructIter::new(of)),
        Err(_) => println!("couldn't open '{}'", filename),
    }

    return None;
}
