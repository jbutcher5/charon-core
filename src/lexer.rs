use crate::models::{Range, Token, WCode, WTokens};
use crate::preprocessor::expand_bracket;
use crate::stdlib::FUNCTIONS;
use lazy_static::lazy_static;
use phf::phf_set;
use rayon::prelude::*;
use regex::Regex;
use substring::Substring;

fn annotate(code: &str, containers: &[String]) -> WTokens {
    lazy_static! {
        static ref SPECIALS: phf::Set<&'static str> = phf_set! {
            ")",
            "(",
            "}",
            "{"
        };
        static ref FULL: Regex = Regex::new(r"(\d+)..(\d+)").unwrap();
        static ref FROM: Regex = Regex::new(r"..(\d+)").unwrap();
        static ref TO: Regex = Regex::new(r"(\d+)..").unwrap();
        static ref EXACT: Regex = Regex::new(r"(\d+)").unwrap();
    }

    let annotated = code
        .split(' ')
        .collect::<Vec<_>>()
        .par_iter()
        .map(|x| match x.parse::<f64>() {
            Ok(n) => Token::Value(n),
            Err(_) => {
                let cleared = x.chars().filter(|&x| x != '\n').collect::<String>();
                let mut chars = cleared.chars();

                if containers.iter().any(|name| *name == cleared) {
                    Token::Container(cleared)
                } else if cleared.len() > 1 && chars.next().unwrap() == '$' {
                    if let Some(captures) = FULL.captures(&cleared) {
                        let caps: Vec<usize> = [1, 2]
                            .iter()
                            .map(|&x| captures.get(x).unwrap().as_str().parse::<usize>().unwrap())
                            .collect();

                        Token::Parameter(Range::Full(caps[1]..=caps[0]))
                    } else if let Some(captures) = TO.captures(&cleared) {
                        let cap = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();

                        Token::Parameter(Range::From(..cap))
                    } else if let Some(captures) = FROM.captures(&cleared) {
                        let cap = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();

                        Token::Parameter(Range::To(cap..))
                    } else if let Some(captures) = EXACT.captures(&cleared) {
                        let value = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();

                        Token::Parameter(Range::Full(value..=value))
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
    let cleaned = expand_bracket(code.to_string());

    let container_symbols = [" <- ", " <-|", " -> "];

    let mut containers = vec![];

    let mut section_buffer: String = "".to_string();
    let mut sectioned_code: Vec<String> = vec![];

    for line in cleaned.split('\n').filter(|&x| x.trim() != "") {
        let re_result: Vec<bool> = container_symbols.iter().map(|x| line.contains(x)).collect();

        if re_result[1] && section_buffer.len() == 0 {
            section_buffer.push_str(line);
        } else if re_result[2] && section_buffer.len() > 0 {
            section_buffer.push_str(format!("\n{}", line).as_str());
        } else if section_buffer.len() > 0 {
            section_buffer.push_str(format!("\n{}", line).as_str());
            sectioned_code.push(section_buffer);
            section_buffer = String::new();
        } else {
            sectioned_code.push(line.to_string());
        }
    }

    if section_buffer.len() > 0 {
        sectioned_code.push(section_buffer);
    }

    sectioned_code
        .iter()
        .filter(|&x| x.trim() != "" && x != "\n")
        .map(|block| {
            let find_results = container_symbols
                .iter()
                .map(|x| match block.find(x) {
                    Some(start) => Some(start..(start + x.len())),
                    None => None,
                })
                .collect::<Vec<Option<std::ops::Range<usize>>>>();

            let find_slice = find_results.as_slice();

            match find_slice {
                [Some(pos), None, None] => {
                    let container = block[..pos.start].to_string();
                    let code = block[pos.end..].to_string();
                    containers.push(container.clone());

                    let default_case = annotate(&code, &containers);

                    WCode {
                        container: Some(container),
                        cases: None,
                        default_case,
                    }
                }
                [None, Some(match_begin), _] => {
                    let container = block[..match_begin.start].to_string();
                    containers.push(container.clone());

                    let mut cases: Vec<String> = block[match_begin.end..]
                        .split('\n')
                        .filter(|&x| x.trim() != "" && x != "\n")
                        .map(|x| String::from(x.trim()))
                        .filter(|x| !x.contains(container_symbols[1]))
                        .collect();

                    let default: String = cases.pop().unwrap();

                    let other_cases: Vec<(WTokens, WTokens)> = cases
                        .iter()
                        .map(|x| {
                            let sep: Vec<WTokens> = x
                                .split(container_symbols[2])
                                .map(|y| annotate(y, &containers))
                                .collect();

                            (sep[0].clone(), sep[1].clone())
                        })
                        .collect();

                    WCode {
                        container: Some(container),
                        cases: Some(other_cases),
                        default_case: annotate(&default, &containers),
                    }
                }
                _ => WCode {
                    container: None,
                    cases: None,
                    default_case: annotate(block, &containers),
                },
            }
        })
        .collect()
}
