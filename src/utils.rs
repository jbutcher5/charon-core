use crate::models::{Range, State, Token, WFuncVariant, WTokens};

pub trait Utils {
    fn get_par(&mut self, n: usize) -> WTokens;
    fn as_nums(&self) -> Vec<f64>;
    fn last_function(&self) -> Option<(usize, WFuncVariant)>;
    fn bundle_groups(&mut self) -> WTokens;
    fn special_pairs(&self, tokens: (String, String), initial_pos: &usize) -> Option<usize>;
    fn first_special_instance(&self, special: String) -> Option<usize>;
    fn skin_content(&mut self);
    fn literal(&self) -> String;
}

impl Token {
    fn is_string(&self) -> Option<String> {
        match self {
            Token::Group(contents) => {
                let mut result = String::new();

                for token in contents {
                    match token {
                        Token::Char(y) => result.push_str(&y.to_string()),
                        _ => return None,
                    }
                }

                Some(result)
            }
            _ => None,
        }
    }
}

impl Utils for WTokens {
    fn get_par(&mut self, n: usize) -> WTokens {
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

    fn as_nums(&self) -> Vec<f64> {
        self.iter()
            .map(|value| match value.clone() {
                Token::Value(n) => n,
                _ => 1.0,
            })
            .collect()
    }

    fn last_function(&self) -> Option<(usize, WFuncVariant)> {
        let reversed = self.iter().rev();

        let mut results: Option<(usize, WFuncVariant)> = None;

        for (i, token) in reversed.enumerate() {
            match token {
                Token::Function(value) => {
                    results = Some((self.len() - (i + 1), WFuncVariant::Function(*value)))
                }
                Token::Container(value) => {
                    results = Some((
                        self.len() - (i + 1),
                        WFuncVariant::Container(value.to_string()),
                    ))
                }
                _ => continue,
            }
        }

        results
    }

    fn bundle_groups(&mut self) -> WTokens {
        let first = self.first_special_instance("{".to_string());
        let second = match first {
            Some(initial_pos) => {
                self.special_pairs(("{".to_string(), "}".to_string()), &initial_pos)
            }
            None => None,
        };

        match (first, second) {
            (Some(x), Some(y)) => {
                let token_group = Token::Group(self[x + 1..y].to_vec().bundle_groups());
                self.splice(x..y + 1, vec![token_group]);
                match self.first_special_instance("{".to_string()) {
                    Some(_) => self.bundle_groups(),
                    None => self.to_owned(),
                }
            }
            (None, None) => self.to_owned(),
            _ => panic!("Invalid grouping!"),
        }
    }

    fn special_pairs(&self, tokens: (String, String), initial_pos: &usize) -> Option<usize> {
        let mut counter = 0;
        let mut next_open = 0;

        for (i, token) in self[*initial_pos + 1..].iter().enumerate() {
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

        for (i, token) in self[next_open..].iter().enumerate() {
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

    fn first_special_instance(&self, special: String) -> Option<usize> {
        for (i, token) in self.iter().enumerate() {
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

    fn skin_content(&mut self) {
        if matches!(
            (self.first(), self.last()),
            (Some(Token::Special(_)), Some(Token::Special(_)))
        ) {
            let bracket_acc = self.iter().fold(0, |acc, x| {
                if matches!(x, Token::Special(y) if y == "(") {
                    acc + 1
                } else if matches!(x, Token::Special(y) if y == ")") {
                    acc - 1
                } else {
                    acc
                }
            });

            if bracket_acc == 0 {
                self.remove(0);
                self.pop();
            }
        }
    }

    fn literal(&self) -> String {
        fn convert(token: &Token) -> String {
            match token {
                Token::Value(x) => x.to_string(),
                Token::Atom(x)
                | Token::Special(x)
                | Token::Container(x)
                | Token::ContainerLiteral(x) => x.to_string(),
                Token::Group(contents) => match token.is_string() {
                    Some(x) => x,
                    _ => format!("{{{}}}", contents.literal()),
                },
                _ => format!("{:?}", token),
            }
        }

        self.iter()
            .fold(String::new(), |acc, x| format!("{} {}", acc, convert(x)))
            .trim()
            .to_string()
    }
}

pub fn as_wcode(arr: Vec<f64>) -> WTokens {
    arr.iter().map(|&value| Token::Value(value)).collect()
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
