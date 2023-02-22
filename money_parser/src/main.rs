// The `use` keyword imports items for other crates or modules.
use std::{env, process};

fn main() {
    // Let's collect the arguments that this program was started with and store them into vector. A vector is a
    // contiguous growable array. `args` should contains two `String` elements: "target\\debug\\money_parser.exe" and
    // "123.45 €"
    let args: Vec<String> = env::args().collect();

    // Let's fail early in case the user forgot to pass a string containing a money representation
    if args.len() < 2 {
        // `eprintln!()` is the same macro than `println!()` except that it writes to stderr
        eprintln!("Usage: money_parser.exe <value_to_parse>");

        // Exit the process and return an error code to the Operating System
        process::exit(1);
    }

    // `println!()` and `eprintln!()` macros are able to format the string and do variable interpolation. Each
    // placeholder is substituted with the variable taken from the list in the same order.
    // Note that we specify a special formatting parameter to display debug information about the variable.
    println!("Args: {:?}", args);
}
