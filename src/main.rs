mod utils;
mod lexer;
mod parser;
mod reducer;

use parser::{Term, parse};
use reducer::{reduce_cbv, reduce_cbn};

fn evaluate(reduce_func: fn(&Term) -> Option<Term>, t: Term) {
    println!("{}", t);
    let mut current = t;
    while let Some(next) = reduce_func(&current) {
        println!(" ==> \n\n{}", next);
        current = next;
    }
    println!(" =/=>\n");
}

fn main() {
    use std::io::{self, Write};
    
    println!("Welcome to the Lambda Calculus REPL!");
    println!("Type your lambda expressions below, or type ':quit' to exit.");

    loop {
        print!("\nλ> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input");
            continue;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }
        
        if input == ":quit" {
            println!("Goodbye!");
            break;
        }

        // We use catch_unwind so that parse() panics don't crash the REPL loop
        let result = std::panic::catch_unwind(|| {
            parse(input)
        });

        match result {
            Ok(ast) => {
                println!("\nEvaluating (Call-by-Name):");
                evaluate(reduce_cbn, ast);
            },
            Err(_) => println!("Parse Error: Please check your syntax and try again."),
        }
    }
}