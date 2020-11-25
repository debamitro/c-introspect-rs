use std::fs::File;
use std::io::BufReader;
use std::vec::Vec;

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

pub fn parse_c_file(filename: &str) -> Option<Vec<C_Struct>> {
    let f = File::open(filename);
    match f {
        Ok(of) => {
            let buf_reader = BufReader::new(of);
            let mut itr = TokenItr::new(buf_reader);

            let parsed_struct = match itr.next() {
                Some(t) => match t {
                    Token::STRUCT => parse_struct(itr),
                    _ => None,
                },
                _ => None,
            };

            match parsed_struct {
                Some(c_struct) => {
                    return Some(vec![c_struct]);
                }
                None => (),
            }
        }
        Err(_) => println!("couldn't open '{}'", filename),
    }

    return None;
}
