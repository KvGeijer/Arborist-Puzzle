mod preprocessor;
mod tokenizer;
mod interpreter;

fn main() {
    // Get the name of the file
    let name = "../ex-arithmetic";

    // Get the contents of the file
    let contents = std::fs::read_to_string(name)
        .expect("Could not find file");

    let preprocessed = preprocessor::preprocess(&contents);    
    println!("PROGRAM:\n{}", preprocessed);

    let tokens = tokenizer::tokenize(preprocessed);
    for token in tokens.iter().take(10) {
        println!("{:?}", token);
    }

    interpreter::interpret(tokens)
}