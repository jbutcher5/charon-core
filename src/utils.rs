use crate::{
    FunctionParameter,
    HashMap,
    Token,
    WCode,
    evaluate,
    WFuncVariant
};

type WFuncPair = (Option<(usize, WFuncVariant)>, Option<(usize, WFuncVariant)>);

pub fn as_nums(arr: WCode) -> Vec<f64> {
    arr.iter()
        .map(|value| match value.clone() {
            Token::Value(n) => n,
            _ => 1.0,
        })
        .collect()
}

pub fn as_wcode(arr: Vec<f64>) -> WCode {
    arr.iter().map(|&value| Token::Value(value)).collect()
}

pub fn outter_function(arr: &WCode) -> WFuncPair {
    let reversed = arr.iter().rev();

    let mut results: WFuncPair = (None, None);

    for (i, token) in arr.iter().enumerate() {
        match token {
            Token::Function(value) => results.0 = Some((i, WFuncVariant::Function(*value))),
            Token::Container(value) => results.0 = Some((i, WFuncVariant::Container(value.to_string()))),
            _ => continue,
        }
    }

    for (i, token) in reversed.enumerate() {
        match token {
            Token::Function(value) => results.1 = Some((arr.len() - (i + 1), WFuncVariant::Function(*value))),
            Token::Container(value) => results.1 = Some((arr.len() - (i + 1), WFuncVariant::Container(value.to_string()))),
            _ => continue,
        }
    }

    results
}

pub fn bracket_pairs(arr: &WCode, initial_pos: &usize) -> Option<usize> {
    let mut counter = 0;
    let mut next_open = 0;

    for (i, token) in arr[initial_pos.clone() + 1..].iter().enumerate() {
        match token {
            Token::Special(value) => {
                if value == ")" {
                    return Some(initial_pos + i + 1);
                } else if value == "(" {
                    next_open = initial_pos + i;
                    counter += 1;
                    break;
                }
            }
            _ => continue,
        }
    }

    for (i, token) in arr[next_open..].iter().enumerate() {
        match token {
            Token::Special(value) => {
                if value == "(" {
                    counter += 1;
                } else if value == ")" {
                    counter -= 1;
                }

                if counter == 0 {
                    return Some(i + next_open);
                }
            }
            _ => continue,
        }
    }

    None
}

pub fn get_first_bracket_open(arr: &WCode) -> Option<usize> {
    for (i, token) in arr.iter().enumerate() {
        match token {
            Token::Special(value) => {
                if value == "(" {
                    return Some(i);
                } else {
                    continue;
                }
            }
            _ => continue,
        }
    }

    None
}

pub fn wfunc(function: &WCode, arr: &WCode, state: &HashMap<String, WCode>) -> WCode {
    let has_remaining_param = function.iter().any(|x| match x {
        Token::Parameter(FunctionParameter::Remaining) => true,
        _ => false,
    });

    let max_param: usize = function.iter().fold(0, |acc, x| match x {
        Token::Parameter(FunctionParameter::Exact(x)) => {
            if acc < x + 1 {
                x + 1
            } else {
                acc
            }
        }
        _ => acc,
    });

    let mut buffer: WCode = Vec::new();

    for token in function {
        match token {
            Token::Parameter(FunctionParameter::Exact(x)) => buffer.push(match arr.get(*x) {
                Some(y) => y.clone(),
                None => continue,
            }),
            Token::Parameter(FunctionParameter::Remaining) => {
                buffer.append(&mut arr[max_param..].to_vec())
            }
            _ => buffer.push(token.clone()),
        }
    }

    let mut result = evaluate(buffer, &state);

    if has_remaining_param {
        result
    } else {
        let mut untouched = arr.clone();
        for _ in 0..max_param {
            untouched.pop();
        }
        untouched.append(&mut result);
        untouched
    }
}
