use crate::models::{FunctionParameter, Token, WCode, WTokens};
use crate::stdlib::FUNCTIONS;
use lazy_static::lazy_static;
use phf::phf_set;
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

    let annotated = code
        .split(' ')
        .map(|x| match x.parse::<f64>() {
            Ok(n) => Token::Value(n),
            Err(_) => {
                let cleared = x.chars().filter(|&x| x != '\n').collect::<String>();
                let mut chars = cleared.chars();

                if containers.iter().any(|name| *name == cleared) {
                    Token::Container(cleared)
                } else if cleared.len() > 1 && chars.nth(0).unwrap() == '$' {
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
    let container_symbols = [" <- ", " <-|", " -> "];

    let mut containers = vec![];

    let mut section_buffer: String = "".to_string();
    let mut sectioned_code: Vec<String> = vec![];

    for line in code.split('\n').filter(|&x| x.trim() != "") {
        let re_result: Vec<bool> = container_symbols.iter().map(|x| line.contains(x)).collect();

        if re_result[1] && section_buffer.len() == 0 {
            section_buffer.push_str(line);
        } else if re_result[2] && section_buffer.len() > 0 {
            section_buffer.push_str(format!("\n{}", line).as_str());
        } else {
            if section_buffer.len() > 0 {
                section_buffer.push_str(format!("\n{}", line).as_str());
                sectioned_code.push(section_buffer);
                section_buffer = String::new();
            } else {
                sectioned_code.push(line.to_string());
            }
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
                    Some(start) => Some(start..x.len()+1),
                    None => None
                })
                .collect::<Vec<Option<std::ops::Range<usize>>>>();

            let find_slice = find_results.as_slice();

            match find_slice {
                [Some(pos), None, None] => {
                    let container = block[..pos.start].to_string();
                    let code = block[pos.end..].to_string();
                    containers.push(container.clone());

                    WCode {
                        container: Some(container),
                        cases: None,
                        default_case: annotate(&code, &containers),
                    }
                }
                [None, Some(match_begin), _] => {
                    let container = block[..match_begin.start].to_string();
                    containers.push(container.clone());

                    let mut cases: Vec<String> = block[match_begin.end..]
                        .split('\n')
                        .map(String::from)
                        .filter(|x| !x.contains(container_symbols[1]))
                        .collect();

                    let default: String = cases.pop().unwrap();

                    let other_cases: Vec<(WTokens, WTokens)> = cases
                        .iter()
                        .map(|x| {
                            let sep: Vec<WTokens> =
                                x.split("->").map(|y| annotate(y, &containers)).collect();

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
