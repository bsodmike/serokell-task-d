extern crate todo_swamp;

use std::io::prelude::*;
use std::io::{self};

use runner::Runner;
use todo_swamp::*;

#[tokio::main]
pub async fn main() {
    let mut tl: TodoList = TodoList::new();
    let stdin = io::stdin();

    let mut runner = Runner {
        reader: io::stdin().lock(),
        writer: io::stdout(),
        log_path: None,
    };

    for line in stdin.lock().lines() {
        if let Ok(l) = line {
            runner.run_line(&l, &mut tl).await;
        }
    }
}
