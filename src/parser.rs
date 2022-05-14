use crate::lexer::LexerToken as LToken;
use crate::models::{State, Token, WCode};
use crate::stdlib::FUNCTIONS;
use logos::{Logos, Span};

pub trait WParser {
    fn parser(&self, code: Vec<(LToken, Span)>, reference: &str) -> Vec<WCode>;
}

impl WParser for State {
    fn parser(&self, code: Vec<(LToken, Span)>, reference: &str) -> Vec<WCode> {
        let mut parsed: Vec<WCode> = vec![];
        let mut current_container = WCode::default();
        let parse = |s: &str| {
            self.parser(LToken::lexer(s).spanned().collect::<Vec<_>>(), s)[0]
                .default_case
                .clone()
        };
        for (token, span) in code {
            if let LToken::Newline = token {
                if current_container != WCode::default() {
                    parsed.push(current_container)
                }

                current_container = WCode::default();
            } else if let LToken::BooleanGuard(name) | LToken::Assignment(name) = token {
                current_container.container = Some(name)
            } else if let LToken::GuardOption((x, y)) = token {
                let mut cases = current_container.cases.unwrap_or_default();
                cases.push((parse(&x), parse(&y)));
                current_container.cases = Some(cases);
            } else if let LToken::GuardDefault(default) = token {
                current_container.default_case = parse(&default);
            } else if let LToken::Token(x) = token {
                current_container.default_case.push(x)
            } else if let LToken::Assignment(name) = token {
                current_container.container = Some(name)
            } else if let LToken::Function(func) = token {
                if let Some(func_reference) = FUNCTIONS.get(&func) {
                    current_container
                        .default_case
                        .push(Token::Function(*func_reference))
                } else {
                    current_container.default_case.push(Token::Container(func))
                }
            } else if let LToken::FunctionLiteral(func) = token {
                if let Some(func_reference) = FUNCTIONS.get(&func) {
                    current_container
                        .default_case
                        .push(Token::FunctionLiteral(*func_reference))
                } else {
                    current_container
                        .default_case
                        .push(Token::ContainerLiteral(func))
                }
            }
        }

        if current_container != WCode::default() {
            parsed.push(current_container)
        }

        parsed
    }
}
