use std::env;
use std::fs;

use russell::frontend::{error, lexer, parser};
use russell::interpreter::treewalk;

fn main() {
    // read the program
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("FATAL ERROR: No file provided.");
    let program = fs::read_to_string(filename).expect("FATAL ERROR: File cannot be read.");

    // lex the program
    let tokens = lexer::lex(&program);
    let errors = error::lex_error::collect_errors(&tokens);
    if errors.len() > 0 {
        for error in errors {
            println!("{}", error::report(&error.into(), &program, filename));
        }
        return;
    }

    // parse the program
    let defns = parser::parse(tokens);

    // interpret the program
    treewalk::interp(defns)
}
