mod cli;
mod password_generator;
mod risk_analyzer;
mod storage;

fn main() {
    let _ = cli::parser::main();
}
