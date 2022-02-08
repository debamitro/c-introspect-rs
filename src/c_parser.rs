//! This module provides a routine called parse_c_file
//! which provides an iterator over all C/C++ structs in a file

use std::fs::File;
use std::io::BufReader;

mod c_tokenizer;
mod c_tokens;

use c_tokenizer::TokenItr;
use c_tokens::{token_value, Token};

#[macro_export]
macro_rules! match_token {
    ( $itr:expr, $target:path ) => {
        if let Some(tok) = $itr.next() {
            if let $target = tok {
                Some($target)
            } else {
                $itr.push_back(tok);
                None
            }
        } else {
            None
        }
    };
}

/// Tries to consume tokens from a `TokenItr`
/// and parse a C variable declaration, like
///
/// ```
/// type1 name1;
/// ```
///
fn parse_declaration(itr: &mut TokenItr) -> Option<C_Declaration> {
    if let Some(tok1) = itr.next() {
        let valid_type = match tok1 {
            Token::INT => true,
            Token::LONG => true,
            Token::IDENTIFIER(_) => true,
            _ => false,
        };

        if !valid_type {
            itr.push_back(tok1);
            return None;
        }

        if let Some(mut tok_id) = itr.next() {
            let mut typename = token_value(tok1);
            if let Token::STAR = tok_id {
                typename.push_str(&token_value(tok_id));
                if let Some(tok3) = itr.next() {
                    tok_id = tok3;
                } else {
                    return None;
                }
            }
            if let Token::IDENTIFIER(_) = tok_id {
                if let Some(_) = match_token!(itr, Token::SEMICOLON) {
                    return Some(C_Declaration {
                        typename: typename,
                        name: token_value(tok_id),
                    });
                }
            } else {
                itr.push_back(tok_id);
            }
        }
    }

    return None;
}

/// Tries to consume tokens from a `TokenItr`
/// and parse a C++ struct declaration, like
///
/// ```
/// struct s1 {
///   type1 field1;
///   type2 field2;
/// };
/// ```
///
/// This function is called after consuming the 'struct'
/// keyword.
/// After successful parsing, a `C_Struct` structure is
/// returned wrapped in an `Option`
fn parse_struct(itr: &mut TokenItr) -> Option<C_Struct> {
    if let Some(tok1 @ Token::IDENTIFIER(_)) = itr.next() {
        if let Some(Token::LBRACE) = itr.next() {
            let mut struct_to_return: C_Struct = C_Struct {
                name: token_value(tok1),
                fields: Vec::<C_Declaration>::new(),
            };

            while let Some(d) = parse_declaration(itr) {
                struct_to_return.fields.push(d);
            }

            if let Some(_) = match_token!(itr, Token::RBRACE) {
                if let Some(_) = match_token!(itr, Token::SEMICOLON) {
                    return Some(struct_to_return);
                }
            }
        }
    }

    return None;
}

/// Tries to consume tokens from a `TokenItr`
/// and parse a C struct declaration, like
///
/// ```
/// typedef struct {
///   type1 field1;
///   type2 field2;
/// } s1;
/// ```
/// or
///
/// ```
/// typedef struct _s1 {
///   type1 field1;
///   type2 field2;
/// } s1;
/// ```
///
/// This function is called after consuming the 'typedef'
/// keyword.
/// After successful parsing, a `C_Struct` structure is
/// returned wrapped in an `Option`
fn parse_typedef_struct(itr: &mut TokenItr) -> Option<C_Struct> {
    if let Some(Token::STRUCT) = itr.next() {
        let mut tok_identifier_or_lbrace = itr.next();
        if let Some(Token::IDENTIFIER(_)) = tok_identifier_or_lbrace {
            tok_identifier_or_lbrace = itr.next();
        }

        if let Some(Token::LBRACE) = tok_identifier_or_lbrace {
            let mut struct_to_return: C_Struct = C_Struct {
                name: String::from(""),
                fields: Vec::<C_Declaration>::new(),
            };

            while let Some(d) = parse_declaration(itr) {
                struct_to_return.fields.push(d);
            }

            if let Some(Token::RBRACE) = itr.next() {
                if let Some(tok1 @ Token::IDENTIFIER(_)) = itr.next() {
                    if let Some(Token::SEMICOLON) = itr.next() {
                        struct_to_return.name = token_value(tok1);
                        return Some(struct_to_return);
                    }
                }
            }
        }
    }

    return None;
}

/// A structure which
/// conveys information about
/// a C/C++ variable as two strings.
/// One for the type and one for the name.
pub struct C_Declaration {
    pub typename: String,
    pub name: String,
}

/// A structure which
/// conveys information about
/// a C/C++ struct, including its name
/// and its fields. Every field
/// of the struct is represented as a `C_Declaration`
/// variable.
pub struct C_Struct {
    pub name: String,
    pub fields: Vec<C_Declaration>,
}

/// An iterator over `C_Struct` values
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

        loop {
            match itr.next() {
                Some(t) => match t {
                    Token::STRUCT => {
                        if let Some(f) = parse_struct(&mut itr) {
                            break Some(f);
                        } else {
                            ()
                        }
                    }
                    Token::TYPEDEF => {
                        if let Some(f) = parse_typedef_struct(&mut itr) {
                            break Some(f);
                        } else {
                            ()
                        }
                    }
                    _ => (),
                },
                None => {
                    self.finished = true;
                    break None;
                }
            }
        }
    }
}

/// This routine takes a file name
/// and returns an `Option` value of type
/// `C_StructIter`
pub fn parse_c_file(filename: &str) -> Option<C_StructIter> {
    let f = File::open(filename);
    match f {
        Ok(of) => return Some(C_StructIter::new(of)),
        Err(_) => println!("couldn't open '{}'", filename),
    }

    return None;
}
