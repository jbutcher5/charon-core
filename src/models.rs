use ariadne::Report;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Value(f64),
    Function(String),
    FunctionLiteral(String),
    Container(String),
    ContainerLiteral(String),
    Parameter(Range),
    Range(Range),
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WFuncVariant {
    Container(String),
    Function(String),
}

pub type WTokens = Vec<Token>;
pub(crate) type WFunc = fn(&State, WTokens) -> Result<WTokens, Vec<Report>>;
pub type State = HashMap<String, Vec<(WTokens, WTokens)>>;
