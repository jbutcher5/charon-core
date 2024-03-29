use charon_ariadne::{Color, Label, Report, ReportBuilder, ReportKind, Source};
use rayon::prelude::*;

use crate::stdlib::{COMPLEX_TYPES, FUNCTIONS};
use crate::{State, Token, Tokens};

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
    fn get_par(
        &mut self,
        func: Token,
        reference_code: Tokens,
        state: &State,
    ) -> Result<Tokens, Report>;
    fn as_nums(&self) -> Vec<f64>;
    fn bundle(&self) -> Tokens;
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

        fn highest_rec(tokens: Tokens) -> usize {
            let mut highest: usize = 0;

            for token in tokens {
                if let Token::Group(inner) | Token::List(inner) = token {
                    let inner_highest = highest_rec(inner);

                    if inner_highest > highest {
                        highest = inner_highest;
                    }
                } else if let Token::Parameter(index) = token {
                    if index + 1 > highest {
                        highest = index + 1
                    }
                }
            }

            highest
        }

        let parameters = match func {
            Token::Function(ident) => FUNCTIONS.get(&ident).unwrap().1.to_vec(),
            Token::ActiveLambda(lambda) => {
                vec!["Any"; highest_rec(lambda)]
            }
            Token::Container(ident) => {
                let container = state.get(&ident).unwrap();

                let all_tokens: Vec<Token> = container
                    .iter()
                    .fold(vec![], |acc, x| [acc, x.0.clone(), x.1.clone()].concat());

                vec!["Any"; highest_rec(all_tokens)]
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

    fn bundle(&self) -> Tokens {
        static BUNDLES: &[(&str, &str, &str); 2] = &[("{", "}", "Group"), ("[", "]", "List")];

        let mut bundled = self.clone();

        for (first, second, collection) in BUNDLES {
            if let Some((x, y)) = self.special_pairs(first, second) {
                let bundled_token = match *collection {
                    "Group" => Token::Group,
                    "List" => Token::List,
                    _ => unimplemented!(),
                }(bundled[x + 1..y].to_vec().bundle());

                bundled.splice(x..y + 1, vec![bundled_token]);
                bundled = bundled.bundle();
            }
        }

        bundled
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
        let reversed: Tokens = arr.iter().cloned().rev().collect();
        function
            .par_iter()
            .map(|token| {
                if let Token::Parameter(index) = token {
                    reversed[*index].clone()
                } else {
                    token.clone()
                }
            })
            .collect()
    }
}
