use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Token {
    Value(f64),
    Function(fn(WTokens) -> WTokens),
    FunctionLiteral(fn(WTokens) -> WTokens),
    Container(String),
    ContainerLiteral(String),
    Parameter(FunctionParameter),
    Atom(String),
    Special(String),
    Group(Vec<Token>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FunctionParameter {
    Exact(usize),
    Remaining,
}

#[derive(Debug, Clone)]
pub struct WCode {
    pub container: Option<String>,
    pub cases: Option<Vec<(WTokens, WTokens)>>,
    pub default_case: WTokens,
}

#[derive(Debug, Clone)]
pub enum WFuncVariant {
    Container(String),
    Function(WFunc),
}

pub type WTokens = Vec<Token>;
pub type WFunc = fn(WTokens) -> WTokens;
pub type State = HashMap<String, WTokens>;
