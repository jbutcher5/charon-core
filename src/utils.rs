use crate::{Token, WCode, WFunc};

type WFuncPair = (Option<(usize, WFunc)>, Option<(usize, WFunc)>);

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
            Token::Function(value) => results.0 = Some((i, *value)),
            _ => continue,
        }
    }

    for (i, token) in reversed.enumerate() {
        match token {
            Token::Function(value) => results.1 = Some((arr.len() - (i + 1), *value)),
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

pub fn get_last_bracket_close(arr: &WCode) -> Option<usize> {
    let reversed = arr.iter().rev();

    for (i, token) in reversed.enumerate() {
        match token {
            Token::Special(value) => {
                if value == ")" {
                    return Some(arr.len() - (i + 1));
                } else {
                    continue;
                }
            }
            _ => continue,
        }
    }

    None
}
