mod preprocessor;
mod interpreter;

fn main() {
    // Get the name of the file
    let name = "../ex-arithmetic";

    // Get the contents of the file
    let contents = std::fs::read_to_string(name)
        .expect("Could not find file");

    let _processed = preprocessor::preprocess(&contents);    
}
