mod modles;
mod stdlib;
mod utils;

use lazy_static::lazy_static;
use phf::phf_set;
use regex::Regex;
use std::collections::HashMap;
use substring::Substring;

use crate::modles::{FunctionParameter, Token, WCode, WFunc, WSection, WFuncVariant};
use crate::stdlib::FUNCTIONS;
use crate::utils::{as_nums, as_wcode, bracket_pairs, get_first_bracket_open, outter_function, wfunc};

fn lexer(code: &str) -> Vec<WSection> {
    lazy_static! {
        static ref RE: Regex = Regex::new(" <- ").unwrap();
        static ref SPECIALS: phf::Set<&'static str> = phf_set! {
            ")",
            "("
        };
    }

    fn inner(code: &str, containers: &Vec<String>) -> WCode {
        code.split(' ')
            .map(|x| match x.parse::<f64>() {
                Ok(n) => Token::Value(n),
                Err(_) => {
                    let cleared = x.chars().filter(|&x| x != '\n').collect::<String>();
                    let mut chars = cleared.chars();

                    if containers.iter().any(|name| *name == cleared) {
                        Token::Container(cleared)
                    } else if cleared.len() > 1 && chars.nth(0).unwrap() == '#' {
                        if let Ok(index) = cleared[1..].parse::<usize>() {
                            Token::Parameter(FunctionParameter::Exact(index))
                        } else if chars.nth(0).unwrap() == 'n' && cleared.len() == 2 {
                            Token::Parameter(FunctionParameter::Remaining)
                        } else {
                            Token::Atom(cleared)
                        }
                    } else if cleared.len() > 2
                        && chars.nth(0).unwrap() == '`'
                        && chars.last().unwrap() == '`'
                    {
                        let function = cleared.substring(1, cleared.len() - 1);

                        Token::FunctionLiteral(
                            *FUNCTIONS
                                .get(function)
                                .unwrap_or_else(|| panic!("Unknown function: {:?}", function)),
                        )
                    } else if SPECIALS.contains(&cleared) {
                        Token::Special(cleared)
                    } else {
                        match FUNCTIONS.get(&cleared) {
                            Some(x) => Token::Function(*x),
                            None => Token::Atom(x.to_string()),
                        }
                    }
                }
            })
            .collect()
    }

    let mut containers = vec![];

    code.split('\n')
        .filter(|&x| x.trim() != "" && x != "\n")
        .map(|line| match RE.find(line) {
            Some(pos) => {

                let container = line[..pos.start()].to_string();
                let code = line[pos.end()..].to_string();

                containers.push(container.clone());
                WSection {
                    container: Some(container),
                    code: inner(&code, &containers),
                }
            }
            None => WSection {
                container: None,
                code: inner(line, &containers),
            },
        })
        .collect()
}

fn section_evaluator(data: Vec<WSection>) -> Vec<WCode> {
    let mut function_map: HashMap<String, WCode> = HashMap::new();
    let mut result: Vec<WCode> = Vec::new();

    for section in data {
        match section.container {
            Some(container) => {
                function_map.insert(container, section.code);
            }
            None => result.push(evaluate(section.code, &function_map)),
        }
    }

    result
}

fn evaluate(data: WCode, state: &HashMap<String, WCode>) -> WCode {
    let mut new_code = data.clone();

    let first = get_first_bracket_open(&new_code);
    let second = match first {
        Some(x) => bracket_pairs(&new_code, &x),
        None => None,
    };

    if first.is_some() && second.is_some() {
        let (x, y) = (first.unwrap(), second.unwrap());
        let bracket_code = &data[x + 1..y];
        new_code.splice(x..y + 1, evaluate(bracket_code.to_vec(), state));

        if get_first_bracket_open(&new_code).is_some() {
            new_code = evaluate(new_code, state)
        }
    }

    let funcs = outter_function(&new_code);

    match funcs {
        (Some((second_func_pos, _)), Some((first_func_pos, func))) => {
            let code_to_evaluate: WCode = new_code[..first_func_pos].to_vec();

            let result = match func {
                WFuncVariant::Function(func) => func(code_to_evaluate),
                WFuncVariant::Container(x) => wfunc(state.get(&x).unwrap(), &code_to_evaluate, state)
            };

            new_code.splice(..first_func_pos + 1, result);

            if first_func_pos != second_func_pos {
                new_code = evaluate(new_code, state);
            }

            new_code
        }
        _ => new_code,
    }
}



fn main() {
    section_evaluator(lexer(
        "
my_sum <- #n sum
( 3 8 ) my_sum OUTPUT
",
    ));
}
