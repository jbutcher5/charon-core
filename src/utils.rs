use charon_ariadne::{Color, Label, Report, ReportBuilder, ReportKind, Source};

use crate::stdlib::{COMPLEX_TYPES, FUNCTIONS};
use crate::{Range, State, Token, Tokens};

pub fn convert(token: &Token) -> String {
    match token {
        Token::Value(x) => x.to_string(),
        Token::Atom(x) => format!(":{}", x),
        Token::Special(x) | Token::Container(x) | Token::Function(x) => x.to_string(),
        Token::Group(contents) => match token.is_string() {
            Some(x) => x,
            _ => format!("{{{}}}", contents.literal()),
        },
        Token::List(contents) => format!("[{}]", contents.literal()),
        Token::FunctionLiteral(x) | Token::ContainerLiteral(x) => format!("`{}`", x),
        _ => format!("{:?}", token),
    }
}

pub fn type_of(token: &Token) -> String {
    let mut buffer = String::new();

    for character in token.to_string().chars() {
        if character == '(' {
            break;
        } else {
            buffer.push(character);
        }
    }

    buffer
}

pub trait Utils {
    fn get_par(&mut self, func: Token, reference_code: Tokens, state: &State) -> Result<Tokens, Report>;
    fn as_nums(&self) -> Vec<f64>;
    fn first_function(&self) -> Option<(std::ops::Range<usize>, Token)>;
    fn bundle_groups(&self) -> Tokens;
    fn bundle_lists(&self) -> Tokens;
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

impl Utils for Tokens {
    fn get_par(
        &mut self,
        func: Token,
        reference_code: Tokens,
        state: &State,
    ) -> Result<Tokens, Report> {
        let mut result = vec![];
        let parameters = match func {
            Token::Function(ident) => FUNCTIONS.get(&ident).unwrap().1.to_vec(),
            Token::Container(ident) => {
                let container = state.get(&ident).unwrap();

                fn highest_rec(tokens: Tokens, max: usize) -> usize {
                    let mut highest = 0;

                    for token in tokens {
                        if let Token::Group(inner) | Token::List(inner) = token {
                            let inner_highest = highest_rec(inner, max);

                            if inner_highest > highest {
                                highest = inner_highest;
                            }
                        } else if let Token::Parameter(range) = token {
                            let range_max = match range {
                                Range::Full(full) => *full.end()+1,
                                Range::From(from) => from.end+1,
                                Range::To(to) => {
                                    if to.start > max {
                                        to.start+1
                                    } else {
                                        max
                                    }
                                }
                            };

                            if range_max > highest {
                                highest = range_max
                            }
                        }
                    }

                    highest
                }

                let all_tokens: Vec<Token> = container
                    .iter()
                    .fold(vec![], |acc, x| [acc, x.0.clone(), x.1.clone()].concat());

                let max: usize = highest_rec(all_tokens, self.len());

                vec!["Any"; max]
            }
            _ => unimplemented!(),
        };
        let literal = reference_code.literal_enumerate();
        let mut final_report: Option<ReportBuilder<std::ops::Range<usize>, Source>> = None;

        for (index, token_type) in parameters.clone().iter().enumerate() {
            match self.pop() {
                Some(content) => {
                    if *token_type == "Any" || type_of(&content) == *token_type {
                        result.push(content);
                        continue;
                    } else if let Some(complex_type) = COMPLEX_TYPES.get(token_type) {
                        if complex_type.contains(&type_of(&content).as_str()) {
                            result.push(content);
                            continue;
                        }
                    }

                    if let Some(report) = final_report {
                        final_report = Some(
                            report.with_label(
                                Label::new(literal.1[literal.1.len() - index - 2].clone())
                                    .with_message(format!(
                                        "This has the type of {} but expected {}.",
                                        type_of(&content),
                                        *token_type
                                    ))
                                    .with_color(Color::Red),
                            ),
                        )
                    } else {
                        final_report = Some(
                            Report::build(ReportKind::Error)
                                .with_message("Mismatched Types")
                                .with_label(
                                    Label::new(literal.1[literal.1.len() - index - 2].clone())
                                        .with_message(format!(
                                            "This has the type of {} but expected {}.",
                                            type_of(&content),
                                            *token_type
                                        ))
                                        .with_color(Color::Red),
                                ),
                        )
                    }
                }
                None => {
                    let mut report = Report::build(ReportKind::Error)
                        .with_message("Missing Parameters")
                        .with_label(
                            Label::new(literal.1[literal.1.len() - 1].clone())
                                .with_message(format!(
                                    "This function expects the parameters ({}).",
                                    parameters
                                        .clone()
                                        .iter()
                                        .fold("".to_string(), |x, acc| format!("{} {}", acc, x))
                                        .trim()
                                ))
                                .with_color(Color::Red),
                        );

                    if !result.is_empty() {
                        report = report.with_label(
                            Label::new(
                                literal.1[0].clone().start
                                    ..literal.1[literal.1.len() - 2].clone().end,
                            )
                            .with_message(format!("Only {} parameter(s) provided.", result.len()))
                            .with_color(Color::Yellow),
                        )
                    }

                    final_report = Some(report)
                }
            }
        }

        if let Some(report) = final_report {
            return Err(report.with_source(Source::from(literal.0)).finish());
        }

        Ok(result)
    }

    fn as_nums(&self) -> Vec<f64> {
        self.iter()
            .map(|value| match value.clone() {
                Token::Value(n) => n,
                _ => 1.0,
            })
            .collect()
    }

    fn first_function(&self) -> Option<(std::ops::Range<usize>, Token)> {
        let mut results: Option<(std::ops::Range<usize>, Token)> = None;

        for (i, token) in self.iter().rev().enumerate() {
            if let Token::Function(value) = token {
                results = Some((0..self.len() - (i + 1), Token::Function(value.to_string())));
            } else if let Token::Container(value) = token {
                results = Some((0..self.len() - (i + 1), Token::Container(value.to_string())));
            } else if let Token::ActiveLambda(lambda) = token {
                results = Some((
                    0..self.len() - (i + 1),
                    Token::ActiveLambda(lambda.to_vec()),
                ));
            }
        }

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

    fn bundle_groups(&self) -> Tokens {
        match self.special_pairs("{", "}") {
            Some((x, y)) => {
                let mut bundled = self.clone();

                let token_group =
                    Token::Group(bundled[x + 1..y].to_vec().bundle_groups().bundle_lists());
                bundled.splice(x..y + 1, vec![token_group]);
                bundled.bundle_groups()
            }
            None => self.to_owned(),
        }
    }

    fn bundle_lists(&self) -> Tokens {
        let pairs = self.special_pairs("[", "]");

        match pairs {
            Some((x, y)) => {
                let mut bundled = self.clone();

                let token_list =
                    Token::List(bundled[x + 1..y].to_vec().bundle_lists().bundle_groups());
                bundled.splice(x..y + 1, vec![token_list]);
                bundled.bundle_lists()
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

        if count == 0 && second_index.is_none() {
            second_index = Some(self[first_index? + 1..].len() + first_index?);
        }

        Some((first_index?, second_index?))
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
                token_range.start = acc.0.len() + 1;
                acc.0 = format!("{} {}", acc.0, token_string);
                token_range.end += token_range.start;
                acc.1.push(token_range);
            }

            acc
        })
    }
}

pub fn encode_string(string: &str) -> Token {
    Token::Group(string.chars().map(Token::Char).collect::<Vec<_>>())
}

pub trait Function {
    fn resolve(&self, function: &Tokens, arr: &Tokens) -> Tokens;
}

impl Function for State {
    fn resolve(&self, function: &Tokens, arr: &Tokens) -> Tokens {
        let mut buffer = vec![];
        let reversed: Tokens = arr.iter().cloned().rev().collect();
        for token in function {
            match token {
                Token::Parameter(range) => {
                    let mut slice = match range {
                        Range::From(from) => {
                            reversed[*from].iter().cloned().rev().collect::<Tokens>()
                        }
                        Range::To(to) => reversed[to.clone()]
                            .iter()
                            .cloned()
                            .rev()
                            .collect::<Tokens>(),
                        Range::Full(full) => reversed[full.clone()]
                            .iter()
                            .cloned()
                            .rev()
                            .collect::<Tokens>(),
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
