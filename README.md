# c-introspect-rs
A type introspection library for C/C++, written in Rust. This is hugely incomplete as of now.

## Dependencies

None. The goal is to recognize as much of C/C++ syntax as needed, and keep everything in Rust.

## Example

There is one example in examples/int_and_long that partially works. You can build it using cargo and run it using the examples/int_and_long/test.h file as an argument. This example will spit out a dumping routine for the C structure.
