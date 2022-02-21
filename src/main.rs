use std::io::{stdin, stdout, Write};

use types::Computer;

mod types;

fn main() -> Result<(), String> {
    let mut args = std::env::args();
    args.next();
    match args.next() {
        Some(program) => {
            let tape = std::fs::read_to_string(program).expect("File not found");
            if let Err(e) = Computer::new(tape.as_str()).run() {
                println!("Error: {}", e)
            }
            Ok(())
        },
        None => {
            loop {
                print!("Enter program\n>>> ");
                stdout().flush().unwrap();
                let input = stdin();
                let mut tape = String::new();
                input.read_line(&mut tape).map_err(|e| e.to_string())?;
                if let Err(e) = Computer::new(tape.trim()).run() {
                    println!("Error: {}", e)
                }
                println!();
            }
        },
    }
}
