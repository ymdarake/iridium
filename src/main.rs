#[macro_use]
extern crate nom;

pub mod assembler;
pub mod instruction;
pub mod repl;
pub mod vm;

fn main() {
    let mut repl = repl::REPL::new();
    let stdin = std::io::stdin();
    let reader = stdin.lock();
    let writer = std::io::stdout();
    repl.run(reader, writer);
}
