mod preprocessor;
mod tokenizer;
mod interpreter;
mod simple_interpreter;
mod parser;
mod generator;

use clap::Parser;
use std::io::Write;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    file: String,

    #[arg(long, default_value_t = false)]
    save: bool,

    #[arg(long, default_value_t = false)]
    simple: bool,
}

fn main() {
    let args = Args::parse();
    let contents = std::fs::read_to_string(&args.file)
        .expect("Could not find file");

    let preprocessed = preprocessor::preprocess(&contents);    

    let tokens = tokenizer::tokenize(preprocessed);

    let syntax_tree = parser::parse(tokens).expect("Could not parse!");

    if args.simple {
        println!("{}", simple_interpreter::interpret(&syntax_tree));
    } else {
        interpreter::interpret(&syntax_tree);
    }

    if args.save {
        let mut output = std::fs::File::create(args.file + ".out").unwrap();
        write!(output, "{}", generator::generate(&syntax_tree)).unwrap();
    }
}
