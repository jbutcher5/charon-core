use crate::lexer::{macros, LexerToken};
use crate::parser::Parser;
use crate::stdlib::FUNCTIONS;
use crate::utils::{Function, Utils};
use crate::{CodeBlock, State, Token, Tokens};
use std::collections::VecDeque;

use charon_ariadne::Report;
use logos::Logos;

pub trait Evaluate {
    fn apply(&mut self, code: &str) -> Result<Vec<Tokens>, Vec<Report>>;
    fn codeblock_eval(&mut self, data: Vec<CodeBlock>) -> Result<Vec<Tokens>, Report>;
    fn eval(&mut self, data: Tokens) -> Result<Tokens, Report>;
}

impl Evaluate for State {
    fn apply(&mut self, code: &str) -> Result<Vec<Tokens>, Vec<Report>> {
        let cleaned_code = macros(code.to_string());
        let lex = LexerToken::lexer(&cleaned_code);

        let parse = self.parser(lex.spanned().collect::<Vec<_>>(), code);

        if let Ok(parsed) = parse {
            Ok(match self.codeblock_eval(parsed) {
                Ok(x) => x,
                Err(reports) => return Err(vec![reports]),
            })
        } else {
            Err(parse.unwrap_err())
        }
    }

    fn codeblock_eval(&mut self, data: Vec<CodeBlock>) -> Result<Vec<Tokens>, Report> {
        let mut result: Vec<Tokens> = Vec::new();

        for codeblock in data {
            match codeblock.container {
                Some(container) => {
                    let mut cases = vec![];

                    if let Some(container_cases) = codeblock.cases {
                        cases.append(&mut container_cases.clone())
                    }

                    cases.push((vec![Token::Value(1.0)], codeblock.default_case));

                    self.insert(container, cases);
                }
                None => result.push(self.eval(codeblock.default_case)?),
            }
        }

        Ok(result)
    }

    fn eval(&mut self, data: Tokens) -> Result<Tokens, Report> {
        let mut execution_stack: VecDeque<Token> = VecDeque::from(data);
        let mut parameter_stack: VecDeque<Token> = VecDeque::new();

        while let Some(token) = execution_stack.pop_front() {
            match token {
                Token::Function(ref ident) => {
                    let parameters = Vec::from(parameter_stack.clone()).get_par(
                        token.clone(),
                        [
                            Vec::from(parameter_stack.clone()),
                            Vec::from(execution_stack.clone()),
                        ]
                        .concat(),
                        self,
                    )?;

                    let mut n = parameters.clone().len();
                    while n > 0 {
                        parameter_stack.pop_back();
                        n -= 1;
                    }

                    let result = FUNCTIONS.get(ident).unwrap().0(self, parameters)?;

                    execution_stack.push_front(result);
                }
                Token::Container(ref ident) => {
                    let parameters = Vec::from(parameter_stack.clone()).get_par(
                        token.clone(),
                        [
                            Vec::from(parameter_stack.clone()),
                            Vec::from(execution_stack.clone()),
                        ]
                        .concat(),
                        self,
                    )?;

                    let mut n = parameters.clone().len();
                    while n > 0 {
                        parameter_stack.pop_back();
                        n -= 1;
                    }


                    let cases = self.get(ident.as_str()).unwrap();

                    let mut selected_consequent: Option<&Vec<Token>> = None;

                    for (predictate, consequent) in cases {
                        if self.clone().eval(self.resolve(predictate, &parameters))?
                            == vec![Token::Value(1.0)]
                        {
                            selected_consequent = Some(consequent);
                            break;
                        }
                    }

                    execution_stack = VecDeque::from(
                        [
                            self.resolve(selected_consequent.unwrap(), &parameters),
                            Vec::from(execution_stack.clone()),
                        ]
                        .concat(),
                    );
                }
                Token::ActiveLambda(ref lambda) => {
                     let parameters = Vec::from(parameter_stack.clone()).get_par(
                        token.clone(),
                        [
                            Vec::from(parameter_stack.clone()),
                            Vec::from(execution_stack.clone()),
                        ]
                        .concat(),
                        self,
                    )?;

                    let mut n = parameters.clone().len();
                    while n > 0 {
                        parameter_stack.pop_back();
                        n -= 1;
                    }

                    execution_stack = VecDeque::from(
                        [
                            self.resolve(&lambda, &parameters),
                            Vec::from(execution_stack.clone()),
                        ]
                        .concat(),
                    );
                }
                Token::Group(contents) => {
                    parameter_stack.push_back(Token::Group(self.eval(contents)?))
                }
                Token::Void => continue,
                _ => parameter_stack.push_back(token),
            }
        }

        Ok(Vec::from(parameter_stack))
    }
}
