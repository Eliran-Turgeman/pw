mod cli;
mod storage;
mod risk_analyzer;

fn main() {
    let _ = cli::parser::main();
}