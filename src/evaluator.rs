use crate::lexer::{macros, LexerToken};
use crate::models::{Range, State, Token, Tokens, WCode, WFuncVariant};
use crate::parser::WParser;
use crate::stdlib::FUNCTIONS;
use crate::utils::{Utils, WFunc};
use itertools::Itertools;

use charon_ariadne::Report;
use logos::Logos;

pub trait WEval {
    fn apply(&mut self, code: &str) -> Result<Vec<Tokens>, Vec<Report>>;
    fn wsection_eval(&mut self, data: Vec<WCode>) -> Result<Vec<Tokens>, Report>;
    fn eval(&mut self, data: Tokens) -> Result<Tokens, Report>;
    fn dissolve(
        &mut self,
        code: &mut Tokens,
        func: WFuncVariant,
        argument_range: &std::ops::Range<usize>,
        arr: Tokens,
    ) -> Result<(), Report>;
}

impl WEval for State {
    fn apply(&mut self, code: &str) -> Result<Vec<Tokens>, Vec<Report>> {
        let cleaned_code = macros(code.to_string());
        let lex = LexerToken::lexer(&cleaned_code);

        let parse = self.parser(lex.spanned().collect::<Vec<_>>(), code);

        if let Ok(parsed) = parse {
            Ok(match self.wsection_eval(parsed) {
                Ok(x) => x,
                Err(reports) => return Err(vec![reports]),
            })
        } else {
            Err(parse.unwrap_err())
        }
    }

    fn wsection_eval(&mut self, data: Vec<WCode>) -> Result<Vec<Tokens>, Report> {
        let mut result: Vec<Tokens> = Vec::new();

        for section in data {
            match section.container {
                Some(container) => {
                    let mut cases = vec![];

                    if let Some(container_cases) = section.cases {
                        cases.append(&mut container_cases.clone())
                    }

                    cases.push((vec![Token::Value(1.0)], section.default_case));

                    self.insert(container, cases);
                }
                None => result.push(match self.eval(section.default_case) {
                    Ok(x) => x,
                    Err(report) => return Err(report),
                }),
            }
        }

        Ok(result)
    }

    fn eval(&mut self, mut data: Tokens) -> Result<Tokens, Report> {
        let mut group_eval: Tokens = vec![];

        while !data.is_empty() {
            let token = data.remove(0);

            if let Token::Group(content) = token {
                match self.eval(content.to_vec()) {
                    Ok(x) => group_eval.push(Token::Group(x)),
                    Err(report) => return Err(report),
                }
            } else {
                group_eval.push(token)
            }
        }

        let mut new_code: Tokens = group_eval.clone();

        while let Some((argument_range, func)) = new_code.first_function() {
            if let Some((x, y)) = new_code.special_pairs("(", ")") {
                let result = new_code[x + 1..y].to_vec();
                new_code.splice(
                    x..=y,
                    match self.eval(result) {
                        Ok(x) => x,
                        Err(report) => return Err(report),
                    },
                );
            } else {
                let code_to_evaluate: Tokens = new_code[argument_range.clone()].to_vec();
                let dissolve =
                    self.dissolve(&mut new_code, func, &argument_range, code_to_evaluate);

                if let Err(report) = dissolve {
                    return Err(report);
                }
            }
        }

        Ok(new_code)
    }

    fn dissolve(
        &mut self,
        code: &mut Tokens,
        func: WFuncVariant,
        argument_range: &std::ops::Range<usize>,
        mut arr: Tokens,
    ) -> Result<(), Report> {
        match func {
            WFuncVariant::ActiveLambda(x) => {
                self.insert(
                    "%lambda".to_string(),
                    vec![(vec![Token::Value(1.0)], x.to_vec())],
                );
                self.dissolve(
                    code,
                    WFuncVariant::Container("%lambda".to_string()),
                    argument_range,
                    arr,
                )
            }
            WFuncVariant::Function(func) => {
                let function_range = argument_range.start..argument_range.end + 1;
                let reference_code: Tokens = code.clone()[function_range.clone()].to_vec();
                let parameters = arr.get_par(&func, reference_code)?;

                let result = match FUNCTIONS.get(&func).unwrap().0(self, parameters) {
                    Ok(result) => result,
                    Err(report) => return Err(report),
                };

                let combined = [arr.clone(), result].concat();

                code.splice(function_range, combined);
                Ok(())
            }
            WFuncVariant::Container(x) => {
                let mut case: Tokens = vec![];
                let mut container_acc: Tokens = vec![];

                for container_case in self.get(&x).unwrap().clone() {
                    container_acc.append(&mut container_case.1.clone());
                    let case_prefix = match self.eval(self.resolve(&container_case.0, &arr)) {
                        Ok(x) => x,
                        Err(report) => return Err(report),
                    };

                    if case_prefix[0] != Token::Value(0.0) {
                        case = container_case.1.clone();
                        break;
                    }
                }

                let expanded_range = container_acc
                    .iter()
                    .fold(vec![], |mut acc, x| {
                        if let Token::Group(mut contents) = x.clone() {
                            acc.append(&mut contents);
                            acc
                        } else {
                            acc.push(x.clone());
                            acc
                        }
                    })
                    .iter()
                    .filter(|x| matches!(x, Token::Parameter(_)))
                    .flat_map(|range| match range {
                        Token::Parameter(Range::Full(full)) => full.clone().collect::<Vec<_>>(),
                        Token::Parameter(Range::From(from)) => (0..=from.end).collect::<Vec<_>>(),
                        Token::Parameter(Range::To(to)) => {
                            (to.start..=arr.len() - 1).collect::<Vec<_>>()
                        }
                        _ => panic!(),
                    })
                    .unique()
                    .map(|wlang_index| arr.len() - (wlang_index + 1))
                    .sorted()
                    .rev()
                    .collect::<Vec<usize>>();

                let result = self.resolve(&case, &arr);
                code.splice(argument_range.end..=argument_range.end, result);

                for n in expanded_range {
                    code.remove(n);
                }

                Ok(())
            }
        }
    }
}
