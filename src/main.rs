use std::io::{self, Write};

use db_engine::database::Database;

fn main() {
    let mut db = Database::new();
    println!("🚀 In-Memory Database REPL. Type 'exit' to quit.");
    loop {
        print!("db > ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let trimmed = input.trim();
        if trimmed == "exit" {
            println!("Bye!");
            break;
        }

        if trimmed.is_empty() {
            continue;
        }

        match db_engine::command::parse_statement(trimmed) {
            Ok(cmd) => {
                if let Err(e) = db.execute(cmd) {
                    println!("❌ Execution error: {}", e);
                }
            }
            Err(e) => println!("❌ Parse error: {}", e),
        }
    }
}
