use crate::models::{Range, State, Token, WCode, WFuncVariant, WTokens};
use crate::parser::WParser;
use crate::utils::{Utils, WFunc};
use itertools::Itertools;

pub trait WEval {
    fn apply(&mut self, code: &str) -> Vec<WTokens>;
    fn wsection_eval(&mut self, data: Vec<WCode>) -> Vec<WTokens>;
    fn eval(&self, data: WTokens) -> WTokens;
    fn dissolve(&self, code: &mut WTokens, func: WFuncVariant, first_func_pos: usize, arr: WTokens);
}

impl WEval for State {
    fn apply(&mut self, code: &str) -> Vec<WTokens> {
        self.wsection_eval(self.parser(code))
    }

    fn wsection_eval(&mut self, data: Vec<WCode>) -> Vec<WTokens> {
        let mut result: Vec<WTokens> = Vec::new();

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
                None => result.push(self.eval(section.default_case)),
            }
        }

        result
    }

    fn eval(&self, data: WTokens) -> WTokens {
        let mut new_code = data;

        while let Some((first_func_pos, func)) = new_code.last_function() {
            if let Some((x, y)) = new_code.special_pairs("(", ")") {
                let mut result = new_code[x + 1..y].to_vec();
                if let Some((first_func_pos, func)) = result.last_function() {
                    let code_to_evaluate = result[..first_func_pos].to_vec();
                    self.dissolve(&mut result, func, first_func_pos, code_to_evaluate);
                    result.skin_content();
                    new_code.splice(x + 1..y, result);
                } else {
                    new_code.splice(x..=y, result);
                }
            } else {
                let code_to_evaluate: WTokens = new_code[..first_func_pos].to_vec();
                self.dissolve(&mut new_code, func, first_func_pos, code_to_evaluate);
            }
        }

        new_code
    }

    fn dissolve(
        &self,
        code: &mut WTokens,
        func: WFuncVariant,
        first_func_pos: usize,
        arr: WTokens,
    ) {
        match func {
            WFuncVariant::Function(func) => {
                let result = func(arr);
                code.splice(..first_func_pos + 1, result);
            }
            WFuncVariant::Container(x) => {
                let mut case: WTokens = vec![];
                let mut container_acc: WTokens = vec![];

                for container_case in self.get(&x).unwrap() {
                    let mut joined = container_case.0.clone();
                    joined.append(&mut container_case.1.clone());
                    container_acc.append(&mut joined);

                    let case_prefix = self.eval(self.apply(&container_case.0, &arr));

                    if case_prefix[0] != Token::Value(0.0) {
                        case = container_case.1.clone();
                        break;
                    }
                }

                let expanded_range = container_acc
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

                let result = self.apply(&case, &arr);
                code.splice(first_func_pos..=first_func_pos, result);
                for n in expanded_range {
                    code.remove(n);
                }
            }
        }
    }
}
