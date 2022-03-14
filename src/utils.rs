use crate::evaluator::WEval;
use crate::models::{FunctionParameter, State, Token, WFuncVariant, WTokens};

type WFuncPair = (Option<(usize, WFuncVariant)>, Option<(usize, WFuncVariant)>);

pub fn as_nums(arr: WTokens) -> Vec<f64> {
    arr.iter()
        .map(|value| match value.clone() {
            Token::Value(n) => n,
            _ => 1.0,
        })
        .collect()
}

pub fn as_wcode(arr: Vec<f64>) -> WTokens {
    arr.iter().map(|&value| Token::Value(value)).collect()
}

pub fn outter_function(arr: &WTokens) -> WFuncPair {
    let reversed = arr.iter().rev();

    let mut results: WFuncPair = (None, None);

    for (i, token) in arr.iter().enumerate() {
        match token {
            Token::Function(value) => results.0 = Some((i, WFuncVariant::Function(*value))),
            Token::Container(value) => {
                results.0 = Some((i, WFuncVariant::Container(value.to_string())))
            }
            _ => continue,
        }
    }

    for (i, token) in reversed.enumerate() {
        match token {
            Token::Function(value) => {
                results.1 = Some((arr.len() - (i + 1), WFuncVariant::Function(*value)))
            }
            Token::Container(value) => {
                results.1 = Some((
                    arr.len() - (i + 1),
                    WFuncVariant::Container(value.to_string()),
                ))
            }
            _ => continue,
        }
    }

    results
}

pub fn bundle_groups(mut arr: WTokens) -> WTokens {
    let first = first_special_instance("{".to_string(), &arr);
    let second = match first {
        Some(initial_pos) => special_pairs(("{".to_string(), "}".to_string()), &arr, &initial_pos),
        None => None,
    };

    match (first, second) {
        (Some(x), Some(y)) => {
            let token_group = Token::Group(arr[x + 1..y].to_vec());
            arr.splice(x..y + 1, vec![token_group]);
            match first_special_instance("{".to_string(), &arr) {
                Some(_) => bundle_groups(arr),
                None => arr,
            }
        }
        (None, None) => arr,
        _ => panic!("Invalid grouping!"),
    }
}

pub fn special_pairs(
    tokens: (String, String),
    arr: &WTokens,
    initial_pos: &usize,
) -> Option<usize> {
    let mut counter = 0;
    let mut next_open = 0;

    for (i, token) in arr[initial_pos.clone() + 1..].iter().enumerate() {
        match token {
            Token::Special(value) => {
                if value == &tokens.1 {
                    return Some(initial_pos + i + 1);
                } else if value == &tokens.0 {
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
                if value == &tokens.0 {
                    counter += 1;
                } else if value == &tokens.1 {
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

pub fn first_special_instance(special: String, arr: &WTokens) -> Option<usize> {
    for (i, token) in arr.iter().enumerate() {
        match token {
            Token::Special(value) => {
                if value == &special {
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

pub trait WFunc {
    fn apply(&self, function: &WTokens, arr: &WTokens, all_tokens_used: &WTokens) -> WTokens;
}

impl WFunc for State {
    fn apply(&self, function: &WTokens, arr: &WTokens, all_tokens_used: &WTokens) -> WTokens {
        fn release_groups(arr: &WTokens) -> WTokens {
            let mut released = arr.clone();

            for (i, token) in arr.iter().enumerate() {
                if let Token::Group(group) = token {
                    released.splice(i..i + 1, group.clone());
                    release_groups(&released);
                }
            }

            released
        }

        fn parameters_used(arr: &WTokens) -> usize {
            arr.iter().fold(0, |acc, x| match x {
                Token::Parameter(FunctionParameter::Exact(x)) => {
                    if acc < x + 1 {
                        x + 1
                    } else {
                        acc
                    }
                }
                _ => acc,
            })
        }

        fn map_parameters(
            mut buffer: WTokens,
            function: &WTokens,
            arr: &WTokens,
            max_param: usize,
        ) -> WTokens {
            for token in function {
                match token {
                    Token::Parameter(FunctionParameter::Exact(x)) => {
                        buffer.push(match arr.get(*x) {
                            Some(y) => y.clone(),
                            None => continue,
                        })
                    }
                    Token::Parameter(FunctionParameter::Remaining) => {
                        buffer.append(&mut arr[max_param..].to_vec())
                    }
                    Token::Group(x) => {
                        buffer.push(Token::Group(map_parameters(vec![], x, arr, max_param)))
                    }
                    _ => buffer.push(token.clone()),
                }
            }

            buffer
        }

        let released_function = release_groups(function);

        let has_remaining_param = released_function.iter().any(|x| match x {
            Token::Parameter(FunctionParameter::Remaining) => true,
            _ => false,
        });

        let max_param: usize = parameters_used(&released_function);
        let mut reversed_arr = arr.clone();
        reversed_arr.reverse();

        let buffer = map_parameters(vec![], function, &reversed_arr, max_param);

        let mut result = self.eval(buffer);

        if has_remaining_param {
            result
        } else {
            let mut untouched = arr.clone();

            let total_param = parameters_used(all_tokens_used);

            for _ in 0..total_param {
                untouched.pop();
            }
            untouched.append(&mut result);
            untouched
        }
    }
}
