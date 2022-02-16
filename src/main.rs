mod modles;
mod stdlib;
mod utils;

use phf::phf_set;
use substring::Substring;

use crate::modles::{Token, WCode, WFunc};
use crate::stdlib::FUNCTIONS;
use crate::utils::{
    as_nums, as_wcode, bracket_pairs, outter_function, get_first_bracket_open
};

static SPECIALS: phf::Set<&'static str> = phf_set! {
    ")",
    "("
};

fn evaluate(data: WCode) -> WCode {
    let mut new_code = data.clone();

    let first = get_first_bracket_open(&new_code);
    let second = match first {
        Some(x) => bracket_pairs(&new_code, &x),
        None => None,
    };

    if first.is_some() && second.is_some() {
        let (x, y) = (first.unwrap(), second.unwrap());
        let bracket_code = &data[x + 1..y];
        new_code.splice(x..y + 1, evaluate(bracket_code.to_vec()));

        if get_first_bracket_open(&new_code).is_some() {
            new_code = evaluate(new_code)
        }
    }

    let funcs = outter_function(&new_code);

    match funcs {
        (Some((second_func_pos, _)), Some((first_func_pos, func))) => {
            let code_to_evaluate: WCode = new_code[..first_func_pos].to_vec();
            let result = func(code_to_evaluate);

            new_code.splice(..first_func_pos + 1, result);

            if first_func_pos != second_func_pos {
                new_code = evaluate(new_code);
            }

            new_code
        }
        _ => new_code,
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
    println!("{:#?}", evaluate(lexer("1 2 div 7 mul 9 atom len")));
}
