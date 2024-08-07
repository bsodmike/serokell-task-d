pub mod parser;
pub mod query;
pub mod runner;
pub mod todo_list;

pub use query::*;
pub use todo_list::*;

pub static ENABLE_DEBUG: bool = false;

#[cfg(test)]
mod tests;

pub(crate) mod util {
    pub static YELLOW: (i32, i32, i32) = (250, 189, 47);
    pub static GREEN: (i32, i32, i32) = (184, 187, 38);

    pub fn colored(color: (i32, i32, i32), text: &str) -> String {
        let (r, g, b) = color;
        return format!("\x1B[38;2;{};{};{}m{}\x1B[0m", r, g, b, text);
    }
}
