use std::io;
mod engine;
mod models;
mod parser;
mod storage;
use crate::engine::ExecutionEngine;
use crate::parser::parse_query;
use crate::storage::ArrayCollection;
fn main() {
    println!("UniDB Shell (Rust) - Type 'exit' to quit\n");

    let collection = ArrayCollection::new();
    let mut engine = ExecutionEngine::new(collection);

    let stdin = io::stdin();
    for line in stdin.lines() {
        let input = match line {
            Ok(line) => line.trim().to_string(),
            Err(_) => break,
        };

        if input.is_empty() {
            continue;
        }

        if input == "exit" {
            println!("Goodbye!");
            break;
        }

        match parse_query(&input) {
            Ok(cmd) => {
                let result = engine.execute(cmd);
                println!("{}", result);
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}