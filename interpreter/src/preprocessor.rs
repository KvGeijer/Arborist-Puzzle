use std::collections::HashMap;

const PREDEF_MACROS: [&str; 5] = [
    "LET",
    "IF",
    "PRINT",
    "BLOCK",
    "SUB",
];


pub fn preprocess(content: &str) -> String {
    // Iterate over all lines and applies textual transformations.
    // Continuously replaces macros for each line until none left. Ininite loops discuraged...

    // Three actions can be taken
    // 1. The line starts with a # -> define a new macro
    // 2. Encounter // -> Remove everything until end of line
    // 3. Encounter a macro invocation and replace it with it's valueo

    let mut macros = init_macros();

    let mut program = String::new();
    for raw_line in content.lines() {
        // Remove comments
        let line: &str = 
            if let Some(ind) = raw_line.find("//") {
                &raw_line[..ind]
            } else {
                raw_line
            };

        // Maybe add a macro definition
        if line.get(0..1) == Some(&"#") {
            let (name, body) = line[1..]
                .trim_start()
                .split_once(' ')
                .expect("Must define a body and name for a macro!");
            
            macros.insert(
                name.to_string(),
                body.to_string()
            );
        // Replace macros in the line
        } else {
            // Replace macros
            program.push_str(&process_line(line, &macros));
            program.push_str("\n");
        }
    }

    program
}

fn init_macros() -> HashMap<String, String> {
    // Initialized the hashmap with the preconfiured macros
    let mut macros = HashMap::new();
    for (mac, ind) in PREDEF_MACROS.iter().zip(1..) {
        macros.insert(mac.to_string(), ind.to_string());
    }
    macros
}

fn process_line(line: &str, macros: &HashMap<String, String>) -> String {
    // Start with the simplest thing, loop over all defined macros and check
    // if they are in the line. In that case replace them. Not effient, but easy.

    let mut result = line.to_string();
    let mut cont = true;
    while cont {
        cont = false;
        for (mac, body) in macros.iter() {
            if result.contains(mac) {
                cont = true;
                result = result.replace(mac, body);
            }
        }
    }
    result
}
