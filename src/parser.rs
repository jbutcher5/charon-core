use crate::lexer::LexerToken as LToken;
use crate::stdlib::FUNCTIONS;
use crate::utils::Utils;
use crate::{CodeBlock, State, Token, Tokens};
use charon_ariadne::{Color, Label, Report, ReportKind, Source};
use logos::{Logos, Span};
use rayon::prelude::*;

pub trait Parser {
    fn parser(
        &self,
        code: Vec<(LToken, Span)>,
        reference: &str,
    ) -> Result<Vec<CodeBlock>, Vec<Report>>;
}

impl Parser for State
where
    Tokens: Utils,
{
    fn parser(
        &self,
        mut code: Vec<(LToken, Span)>,
        reference: &str,
    ) -> Result<Vec<CodeBlock>, Vec<Report>> {
        let mut parsed: Vec<CodeBlock> = vec![];
        let mut current_container = CodeBlock::default();
        let parse = |s: &str| match self
            .parser(LToken::lexer(s).spanned().collect::<Vec<_>>(), reference)
        {
            Ok(tokens) => Ok(tokens[0].default_case.clone()),
            Err(errors) => Err(errors),
        };
        let mut errors: Vec<Report> = vec![];

        code = code.iter().fold(vec![], |acc, x| {
            let mut new = acc.clone();

            if acc.is_empty() {
                new.push(x.clone());
            } else if let ((LToken::Error, span), (LToken::Error, span_current)) =
                (acc[acc.len() - 1].clone(), x.clone())
            {
                new[acc.len() - 1] = (
                    LToken::Error,
                    Span {
                        start: span.start,
                        end: span_current.end,
                    },
                )
            } else {
                new.push(x.clone());
            }
            new
        });

        for (token, span) in code {
            if let LToken::Newline = token {
                if current_container != CodeBlock::default() {
                    parsed.push(current_container)
                }

                current_container = CodeBlock::default();
            } else if let LToken::BooleanGuard(name) | LToken::Assignment(name) = token {
                current_container.container = Some(name)
            } else if let LToken::GuardOption((x, y)) = token {
                let mut cases: Vec<(Tokens, Tokens)> = match current_container.cases.clone() {
                    Some(x) => x,
                    _ => vec![],
                };

                match (parse(&x), parse(&y)) {
                    (Ok(token_x), Ok(token_y)) => {
                        cases.push((token_x, token_y));
                        current_container.cases = Some(cases);
                    }
                    (Err(mut result_x), Err(mut result_y)) => {
                        errors.append(&mut result_x);
                        errors.append(&mut result_y)
                    }
                    (Err(mut result_x), Ok(_)) => errors.append(&mut result_x),
                    (Ok(_), Err(mut result_y)) => errors.append(&mut result_y),
                };
            } else if let LToken::GuardDefault(default) = token {
                match parse(&default) {
                    Ok(default_case) => current_container.default_case = default_case,
                    Err(mut results) => errors.append(&mut results),
                }
            } else if let LToken::Token(x) = token {
                current_container.default_case.push(x)
            } else if let LToken::Assignment(name) = token {
                current_container.container = Some(name)
            } else if let LToken::Function(func) = token {
                if FUNCTIONS.get(&func).is_some() {
                    current_container.default_case.push(Token::Function(func))
                } else {
                    current_container.default_case.push(Token::Container(func))
                }
            } else if let LToken::FunctionLiteral(func) = token {
                if FUNCTIONS.get(&func).is_some() {
                    current_container
                        .default_case
                        .push(Token::FunctionLiteral(func))
                } else {
                    current_container
                        .default_case
                        .push(Token::ContainerLiteral(func))
                }
            } else if let LToken::Error = token {
                errors.push(
                    Report::build(ReportKind::Error)
                        .with_message("Unknown Token")
                        .with_label(
                            Label::new(span)
                                .with_message("The parser can not identify this token.")
                                .with_color(Color::Red),
                        )
                        .with_source(Source::from(reference))
                        .finish(),
                )
            }
        }

        if current_container != CodeBlock::default() {
            parsed.push(current_container)
        }

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(parsed
                .par_iter()
                .cloned()
                .map(|code_block| CodeBlock {
                    container: code_block.container,
                    cases: code_block.cases.map(|inner| {
                        inner
                            .iter()
                            .map(|(predictate, consequent)| {
                                (
                                    predictate.bundle_groups().bundle_lists(),
                                    consequent.bundle_groups().bundle_lists(),
                                )
                            })
                            .collect::<Vec<_>>()
                    }),
                    default_case: code_block.default_case.bundle_groups().bundle_lists(),
                })
                .collect::<Vec<_>>())
        }
    }
}
