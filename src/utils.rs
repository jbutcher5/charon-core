use crate::models::{Range, State, Token, WFuncVariant, WTokens};

fn convert(token: &Token) -> String {
    match token {
        Token::Value(x) => x.to_string(),
        Token::Atom(x) | Token::Special(x) | Token::Container(x) | Token::ContainerLiteral(x) => {
            x.to_string()
        }
        Token::Group(contents) => match token.is_string() {
            Some(x) => x,
            _ => format!("{{{}}}", contents.literal()),
        },
        _ => format!("{:?}", token),
    }
}

pub trait Utils {
    fn get_par(&mut self, n: usize) -> WTokens;
    fn as_nums(&self) -> Vec<f64>;
    fn first_function(&self) -> Option<(std::ops::Range<usize>, WFuncVariant)>;
    fn bundle_groups(&mut self) -> WTokens;
    fn special_pairs(&self, first: &str, second: &str) -> Option<(usize, usize)>;
    fn skin_content(&mut self);
    fn literal(&self) -> String;
    fn literal_enumerate(&self) -> (String, Vec<std::ops::Range<usize>>);
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

    fn first_function(&self) -> Option<(std::ops::Range<usize>, WFuncVariant)> {
        let mut results: Option<(std::ops::Range<usize>, WFuncVariant)> = None;

        for (i, token) in self.iter().rev().enumerate() {
            if let Token::Function(value) = token {
                results = Some((
                    0..self.len() - (i + 1),
                    WFuncVariant::Function(value.to_string()),
                ));
            } else if let Token::Container(value) = token {
                results = Some((
                    0..self.len() - (i + 1),
                    WFuncVariant::Container(value.to_string()),
                ));
            }
        }

        results.as_ref()?;

        let mut count: i32 = 0;

        for (i, token) in self[..results.clone()?.0.end].iter().rev().enumerate() {
            if count != 0 {
                if Token::Special("(".to_string()) == *token {
                    count += 1;
                } else if Token::Special(")".to_string()) == *token {
                    count -= 1;
                }
            } else if Token::Special("(".to_string()) == *token {
                results = Some((
                    self.len() - i - 2..results.clone()?.0.end,
                    results.clone()?.1,
                ))
            } else if Token::Special(")".to_string()) == *token {
                count -= 1;
            }
        }

        results
    }

    fn bundle_groups(&mut self) -> WTokens {
        match self.special_pairs("{", "}") {
            Some((x, y)) => {
                let token_group = Token::Group(self[x + 1..y].to_vec().bundle_groups());
                self.splice(x..y + 1, vec![token_group]);
                match self.special_pairs("{", "}") {
                    Some(_) => self.bundle_groups(),
                    None => self.to_owned(),
                }
            }
            None => self.to_owned(),
        }
    }

    fn special_pairs(&self, first: &str, second: &str) -> Option<(usize, usize)> {
        let mut first_index: Option<usize> = None;
        let mut second_index: Option<usize> = None;

        for (index, value) in self.iter().enumerate() {
            if Token::Special(first.to_string()) == *value {
                first_index = Some(index);
                break;
            }
        }

        first_index?;

        let mut count: i32 = 1;

        for (index, value) in self[first_index? + 1..].iter().enumerate() {
            if count == 0 {
                second_index = Some(index + first_index?);
                break;
            } else if Token::Special(first.to_string()) == *value {
                count += 1;
            } else if Token::Special(second.to_string()) == *value {
                count -= 1;
            }
        }

        if let Some(second_index) = second_index {
            Some((first_index?, second_index))
        } else {
            None
        }
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
        self.iter()
            .fold(String::new(), |acc, x| format!("{} {}", acc, convert(x)))
            .trim()
            .to_string()
    }

    fn literal_enumerate(&self) -> (String, Vec<std::ops::Range<usize>>) {
        self.iter().fold((String::new(), vec![]), |mut acc, x| {
            let token_string = convert(x);
            let mut token_range = 0..token_string.len();

            if acc.0.is_empty() {
                acc = (token_string, vec![token_range]);
            } else {
                token_range.start = acc.0.len();
                acc.0 = format!("{} {}", acc.0, token_string);
                token_range.end += token_range.start;
                acc.1.push(token_range);
            }

            acc
        })
    }
}

pub fn as_wcode(arr: Vec<f64>) -> WTokens {
    arr.iter().map(|&value| Token::Value(value)).collect()
}

pub trait WFunc {
    fn resolve(&self, function: &WTokens, arr: &WTokens) -> WTokens;
}

impl WFunc for State {
    fn resolve(&self, function: &WTokens, arr: &WTokens) -> WTokens {
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
                Token::Group(x) => buffer.push(Token::Group(self.resolve(x, arr))),
                _ => buffer.push(token.clone()),
            }
        }

        buffer
    }
}
