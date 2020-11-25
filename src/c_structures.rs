use std::string::String;
use std::vec::Vec;

pub struct C_Declaration {
    pub typename: String,
    pub name: String,
}

pub struct C_Struct {
    pub name: String,
    pub fields: Vec<C_Declaration>,
}
