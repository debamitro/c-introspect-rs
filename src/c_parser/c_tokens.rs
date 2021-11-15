#[derive(Debug)]
pub struct Identifier {
    pub value: String,
}

#[derive(Debug)]
pub enum Token {
    LBRACE,
    RBRACE,
    SEMICOLON,
    INT,
    LONG,
    STRUCT,
    IDENTIFIER(Identifier),
}

pub fn token_value(tok: Token) -> String {
    match tok {
        Token::LBRACE => String::from("{"),
        Token::RBRACE => String::from("}"),
        Token::SEMICOLON => String::from(";"),
        Token::INT => String::from("int"),
        Token::LONG => String::from("long"),
        Token::STRUCT => String::from("struct"),
        Token::IDENTIFIER(id) => id.value,
    }
}
