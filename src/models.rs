use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Value(f64),
    Function(fn(WTokens) -> WTokens),
    FunctionLiteral(fn(WTokens) -> WTokens),
    Container(String),
    ContainerLiteral(String),
    Parameter(Range),
    Atom(String),
    Char(char),
    Special(String),
    Group(Vec<Token>),
}

#[derive(PartialEq, Debug, Default, Clone)]
pub struct WCode {
    pub container: Option<String>,
    pub cases: Option<Vec<(WTokens, WTokens)>>,
    pub default_case: WTokens,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Range {
    Full(std::ops::RangeInclusive<usize>),
    To(std::ops::RangeFrom<usize>),
    From(std::ops::RangeTo<usize>),
}

#[derive(Debug, Clone)]
pub enum WFuncVariant {
    Container(String),
    Function(WFunc),
}

pub type WTokens = Vec<Token>;
pub(crate) type WFunc = fn(WTokens) -> WTokens;
pub type State = HashMap<String, Vec<(WTokens, WTokens)>>;
