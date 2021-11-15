# c-introspect-rs
A type introspection library for C/C++, written in Rust. This is hugely incomplete as of now.

## Dependencies

None. The goal is to recognize as much of C/C++ syntax as needed, and keep everything in Rust.

## Example

There is one example in examples/int_and_long that partially works. You can build and run it using this command:

```
cargo run --example int_and_long -- examples/int_and_long/test.h
```

This example will spit out a dumping routine for a C structure defined in examples/int_and_long/test.h

```
void var_dump_foo (struct foo * var) {
  printf ("struct foo = {\n");
  printf ("  a = %d\n",var->a);
  printf ("  big_one = %ld\n",var->big_one);
  printf ("}\n");
}
```
