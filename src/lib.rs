//! This crate provides a module called c_parser
//! which provids a routine for iterating over C structs

pub mod c_parser;
pub mod c_structures;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
