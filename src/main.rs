mod modles;
mod stdlib;
mod utils;

use phf::phf_set;
use substring::Substring;

use crate::modles::{Token, WCode, WFunc};
use crate::stdlib::FUNCTIONS;
use crate::utils::{
    as_nums, as_wcode, get_first_bracket_open, get_last_bracket_close, last_function,
};

static SPECIALS: phf::Set<&'static str> = phf_set! {
    ")",
    "("
};

fn evaluate(data: WCode) -> WCode {
    let mut new_code = data.clone();

    let brackets = (
        get_first_bracket_open(&new_code),
        get_last_bracket_close(&new_code),
    );

    if brackets.0.is_some() && brackets.1.is_some() {
        let (x, y) = (brackets.0.unwrap(), brackets.1.unwrap());
        let bracket_code = &data[x + 1..y];
        new_code.splice(x..y + 1, evaluate(bracket_code.to_vec()));
    }

    match last_function(&new_code) {
        Some((func_pos, func)) => {
            let code_to_evaluate: WCode = new_code[..func_pos].to_vec();
            let result = func(code_to_evaluate);

            new_code.splice(..func_pos + 1, result);
            new_code
        }
        None => new_code,
    }
}

fn lexer(code: &str) -> WCode {
    code.split(' ')
        .map(|x| match x.parse::<f64>() {
            Ok(n) => Token::Value(n),
            Err(_) => {
                let mut chars = x.chars();

                if x.len() > 2 && chars.next().unwrap() == '`' && chars.last().unwrap() == '`' {
                    let function = x.substring(1, x.len() - 1);

                    Token::FunctionLiteral(
                        *FUNCTIONS
                            .get(function)
                            .unwrap_or_else(|| panic!("Unknown function: {:?}", function)),
                    )
                } else if SPECIALS.contains(x) {
                    Token::Special(x.to_string())
                } else {
                    match FUNCTIONS.get(x) {
                        Some(x) => Token::Function(*x),
                        None => Token::Atom(x.to_string()),
                    }
                }
            }
        })
        .collect()
}

fn main() {
    println!("{:#?}", evaluate(lexer("1 2 3 3 ( ( 4 6 ) + ) sum 8")));
}
