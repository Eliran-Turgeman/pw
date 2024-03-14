mod cli;
mod storage;
mod risk_analyzer;
mod password_generator;

fn main() {
    let _ = cli::parser::main();
}