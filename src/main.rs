mod modles;
mod stdlib;
mod utils;
mod lexer;

use std::collections::HashMap;
use crate::modles::{FunctionParameter, Token, WCode, WFunc, WSection, WFuncVariant};
use crate::utils::{as_nums, as_wcode, bracket_pairs, get_first_bracket_open, outter_function, wfunc};
use crate::lexer::lexer;

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
