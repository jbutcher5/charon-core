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

pub fn function_pairs(arr: &WCode) -> WFuncPair {
    let reversed = arr.iter().rev();

    let mut results: WFuncPair = (None, None);

    for (i, token) in reversed.enumerate() {
        match token {
            Token::Function(value) => results.1 = Some((arr.len() - (i + 1), *value)),
            _ => continue,
        }
    }

    for (i, token) in arr.iter().enumerate() {
        match token {
            Token::Function(value) => results.0 = Some((i, *value)),
            _ => continue
        }
    }

    results
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
