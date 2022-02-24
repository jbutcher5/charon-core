use crate::models::{FunctionParameter, Token, WCode, WTokens};
use crate::stdlib::FUNCTIONS;
use lazy_static::lazy_static;
use phf::phf_set;
use regex::Regex;
use substring::Substring;

fn annotate(code: &str, containers: &Vec<String>) -> WTokens {
    lazy_static! {
        static ref SPECIALS: phf::Set<&'static str> = phf_set! {
            ")",
            "(",
            "}",
            "{"
        };
    }

    let annotated = code.split(' ')
        .map(|x| match x.parse::<f64>() {
            Ok(n) => Token::Value(n),
            Err(_) => {
                let cleared = x.chars().filter(|&x| x != '\n').collect::<String>();
                let mut chars = cleared.chars();

                if containers.iter().any(|name| *name == cleared) {
                    Token::Container(cleared)
                } else if cleared.len() > 1 && chars.nth(0).unwrap() == '#' {
                    if let Ok(index) = cleared[1..].parse::<usize>() {
                        Token::Parameter(FunctionParameter::Exact(index))
                    } else if chars.nth(0).unwrap() == 'n' && cleared.len() == 2 {
                        Token::Parameter(FunctionParameter::Remaining)
                    } else {
                        Token::Atom(cleared)
                    }
                } else if cleared.len() > 2
                    && chars.nth(0).unwrap() == '`'
                    && chars.last().unwrap() == '`'
                {
                    let slice = cleared.substring(1, cleared.len() - 1);

                    if containers.iter().any(|name| *name == slice) {
                        Token::ContainerLiteral(slice.to_string())
                    } else {
                        Token::FunctionLiteral(
                            *FUNCTIONS
                                .get(slice)
                                .unwrap_or_else(|| panic!("Unknown function: {:?}", slice)),
                        )
                    }
                } else if SPECIALS.contains(&cleared) {
                    Token::Special(cleared)
                } else {
                    match FUNCTIONS.get(&cleared) {
                        Some(x) => Token::Function(*x),
                        None => Token::Atom(x.to_string()),
                    }
                }
            }
        })
        .collect();

    crate::utils::bundle_groups(annotated)
}

pub fn lexer(code: &str) -> Vec<WCode> {
    lazy_static! {
        static ref RE: Regex = Regex::new(" <- ").unwrap();
    }

    let mut containers = vec![];

    code.split('\n')
        .filter(|&x| x.trim() != "" && x != "\n")
        .map(|line| match RE.find(line) {
            Some(pos) => {
                let container = line[..pos.start()].to_string();
                let code = line[pos.end()..].to_string();
                containers.push(container.clone());

                WCode {
                    container: Some(container),
                    code: annotate(&code, &containers),
                }
            }
            None => WCode {
                container: None,
                code: annotate(line, &containers),
            },
        })
        .collect()
}
