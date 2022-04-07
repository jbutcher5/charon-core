use crate::models::{Range, State, Token, WFuncVariant, WTokens};

pub trait Utils {
    fn get_par(&mut self, n: usize) -> WTokens;
    fn as_nums(&self) -> Vec<f64>;
    fn last_function(&self) -> Option<(usize, WFuncVariant)>;
    fn bundle_groups(&mut self) -> WTokens;
    fn special_pairs(&self, tokens: (String, String), initial_pos: &usize) -> Option<usize>;
    fn first_special_instance(&self, special: String) -> Option<usize>;
    fn skin_content(&mut self);
}

impl Utils for WTokens {
pub fn get_par(&mut self, n: usize) -> WTokens {
    let mut result = vec![];

    for _ in 0..n {
        result.push(match self.pop() {
            Some(content) => content,
            None => panic!(
                "Too few arguments in {:?} where {} arguments were expected!",
                self, n
            ),
        })
    }

    result
}
}

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


pub fn last_function(arr: &WTokens) -> Option<(usize, WFuncVariant)> {
    let reversed = arr.iter().rev();

    let mut results: Option<(usize, WFuncVariant)> = None;

    for (i, token) in reversed.enumerate() {
        match token {
            Token::Function(value) => {
                results = Some((arr.len() - (i + 1), WFuncVariant::Function(*value)))
            }
            Token::Container(value) => {
                results = Some((
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
            let token_group = Token::Group(bundle_groups(arr[x + 1..y].to_vec()));
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

    for (i, token) in arr[*initial_pos + 1..].iter().enumerate() {
        match token {
            Token::Special(value) => {
                if value == &tokens.1 {
                    return Some(initial_pos + i + 1);
                } else if value == &tokens.0 {
                    next_open = initial_pos + i + 1;
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

pub fn skin_content(arr: &mut WTokens) {
    if matches!(
        (arr.first(), arr.last()),
        (Some(Token::Special(_)), Some(Token::Special(_)))
    ) {
        let bracket_acc = arr.iter().fold(0, |acc, x| {
            if matches!(x, Token::Special(y) if y == "(") {
                acc + 1
            } else if matches!(x, Token::Special(y) if y == ")") {
                acc - 1
            } else {
                acc
            }
        });

        if bracket_acc == 0 {
            arr.remove(0);
            arr.pop();
        }
    }
}

pub trait WFunc {
    fn apply(&self, function: &WTokens, arr: &WTokens) -> WTokens;
}

impl WFunc for State {
    fn apply(&self, function: &WTokens, arr: &WTokens) -> WTokens {
        let mut buffer = vec![];
        let reversed: WTokens = arr.iter().cloned().rev().collect();

        for token in function {
            match token {
                Token::Parameter(range) => {
                    let mut slice = match range {
                        Range::From(from) => {
                            reversed[*from].iter().cloned().rev().collect::<WTokens>()
                        }
                        Range::To(to) => reversed[to.clone()]
                            .iter()
                            .cloned()
                            .rev()
                            .collect::<WTokens>(),
                        Range::Full(full) => reversed[full.clone()]
                            .iter()
                            .cloned()
                            .rev()
                            .collect::<WTokens>(),
                    };

                    buffer.append(&mut slice);
                }
                Token::Group(x) => buffer.push(Token::Group(self.apply(x, arr))),
                _ => buffer.push(token.clone()),
            }
        }

        buffer
    }
}
