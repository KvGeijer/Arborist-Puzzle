mod preprocessor;
mod tokenizer;
mod interpreter;
mod parser;
mod generator;

use clap::Parser;
use std::io::Write;

#[derive(Parser)]
struct Args {
    file: String,
}

fn main() {
    let args = Args::parse();
    let contents = std::fs::read_to_string(&args.file)
        .expect("Could not find file");

    let preprocessed = preprocessor::preprocess(&contents);    
    println!("PROGRAM:\n{}", preprocessed);

    let tokens = tokenizer::tokenize(preprocessed);

    let syntax_tree = parser::parse(tokens).expect("Could not parse!");
    
    interpreter::interpret(&syntax_tree);

    let mut output = std::fs::File::create(args.file + ".out").unwrap();
    write!(output, "{}", generator::generate(&syntax_tree)).unwrap();
}
