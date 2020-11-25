use c_introspect_rs::c_parser::parse_c_file;
use c_introspect_rs::c_structures::C_Struct;
use std::env::args;

fn format_specifier(typename: &str) -> &'static str {
    if typename == "int" {
        return "%d";
    } else if typename == "long" {
        return "%ld";
    }

    return "%x";
}

fn generate_var_dump (filename: &str) {
    if let Some(itr) = parse_c_file(filename) {
        for c_struct in itr {
            println!(
                "void var_dump_{} (struct {} * var) {{",
                c_struct.name, c_struct.name
            );
            println!("  printf (\"struct {} = {{\\n\");", c_struct.name);
            for field in c_struct.fields.iter() {
                println!(
                    "  printf (\"  {} = {}\\n\",var->{});",
                    field.name,
                    format_specifier(&field.typename),
                    field.name
                );
            }
            println!("  printf (\"}}\\n\");");
            println!("}}")
        }
    }
}

fn main() {
    let mut command_args = args();
    command_args.next();        // ignore the first argument
    if let Some(filename) = command_args.next() {
        generate_var_dump (&filename);
    }
}
