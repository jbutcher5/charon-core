use crate::lexer::LexerToken as LToken;
use crate::models::{State, Token, WCode, WTokens};
use crate::stdlib::FUNCTIONS;
use crate::utils::Utils;
use ariadne::{Label, Report, ReportKind};
use logos::{Logos, Span};

pub trait WParser {
    fn parser(&self, code: Vec<(LToken, Span)>) -> Result<Vec<WCode>, Vec<Report>>;
}

impl WParser for State
where
    WTokens: Utils,
{
    fn parser(&self, code: Vec<(LToken, Span)>) -> Result<Vec<WCode>, Vec<Report>> {
        let mut parsed: Vec<WCode> = vec![];
        let mut current_container = WCode::default();
        let parse = |s: &str| match self.parser(LToken::lexer(s).spanned().collect::<Vec<_>>()) {
            Ok(tokens) => Ok(tokens[0].default_case.clone()),
            Err(errors) => Err(errors),
        };
        let mut errors: Vec<Report> = vec![];

        for (token, span) in code {
            if let LToken::Newline = token {
                if current_container != WCode::default() {
                    if let Some(cases) = current_container.cases {
                        current_container.cases = Some(
                            cases
                                .clone()
                                .iter()
                                .map(|(x, y)| {
                                    (x.clone().bundle_groups(), y.clone().bundle_groups())
                                })
                                .collect::<Vec<_>>(),
                        );
                    }

                    current_container.default_case = current_container.default_case.bundle_groups();

                    parsed.push(current_container)
                }

                current_container = WCode::default();
            } else if let LToken::BooleanGuard(name) | LToken::Assignment(name) = token {
                current_container.container = Some(name)
            } else if let LToken::GuardOption((x, y)) = token {
                let mut cases: Vec<(WTokens, WTokens)> = match current_container.cases.clone() {
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
                    Report::build(ReportKind::Error, (), 0)
                        .with_message("Unknown Token")
                        .with_label(Label::new(span).with_message("Unkown Token"))
                        .finish(),
                )
            }
        }

        if current_container != WCode::default() {
            parsed.push(current_container)
        }

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(parsed)
        }
    }
}
