#[derive(Debug, PartialEq)]
pub enum Token {
    LPAR,
    RPAR,
    INT(i32),
}

fn accept_int(string: &[u8], ind: &mut usize) -> Token {
    let mut int = 0;
    let sign = if string[*ind] == b'-' {
        *ind += 1;
        -1
    } else {
        1
    };
    while string[*ind].is_ascii_digit() {
        int = int * 10 + (string[*ind] - b'0') as i32;
        *ind += 1;
    }
    Token::INT(int * sign)
}

fn accept_token(string: &[u8], ind: &mut usize) -> Token {
    match string[*ind] {
        b'(' => {
            *ind += 1;
            Token::LPAR
        }
        b')' => {
            *ind += 1;
            Token::RPAR
        }
        _ => accept_int(string, ind),
    }
}

pub fn tokenize(program: String) -> Vec<Token> {
    let mut ind = 0;
    let mut tokens = vec![];
    let string = program.as_bytes();
    while ind < string.len() {
        if string[ind].is_ascii_whitespace() {
            ind += 1;
        } else {
            tokens.push(accept_token(string, &mut ind))
        }
    }
    tokens
}
