mod modles;
mod stdlib;
mod utils;

use phf::phf_set;
use substring::Substring;
use lazy_static::lazy_static;
use regex::Regex;

use crate::modles::{Token, FunctionParameter, WCode, WFunc, WSection};
use crate::stdlib::FUNCTIONS;
use crate::utils::{as_nums, as_wcode, bracket_pairs, get_first_bracket_open, outter_function};

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

fn lexer(code: &str, containers: Vec<String>) -> WCode {
    code.split(' ')
        .map(|x| match x.parse::<f64>() {
            Ok(n) => Token::Value(n),
            Err(_) => {
                let mut chars = x.chars();

                if containers.iter().any(|&name| name == x) {
                    Token::Container(x.to_string())
                } else if x.len() > 1 && chars.nth(0).unwrap() == '#' {
                    if let Ok(index) = x[1..].parse::<usize>() {
                        Token::Parameter(FunctionParameter::Exact(index))
                    } else if chars.nth(1).unwrap() == 'n' && x.len() == 2 {
                        Token::Parameter(FunctionParameter::Remaining)
                    } else {
                        Token::Atom(x.to_string())
                    }
                } else if x.len() > 2 && chars.nth(0).unwrap() == '`' && chars.last().unwrap() == '`' {
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
    evaluate(lexer("1 2 div 7 mul 9 atom `len` OUTPUT len OUTPUT"));
}
