pub mod evaluator;
mod lexer;
mod parser;
mod stdlib;
mod utils;

use charon_ariadne::Report;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Value(f64),
    Function(String),
    FunctionLiteral(String),
    Container(String),
    ContainerLiteral(String),
    Lambda(Vec<Token>),
    ActiveLambda(Vec<Token>),
    Parameter(Range),
    Atom(String),
    Char(char),
    Special(String),
    Group(Vec<Token>),
    List(Vec<Token>),
    Iterator(Vec<Token>),
    Null,
    Void,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(PartialEq, Debug, Default, Clone)]
pub struct CodeBlock {
    pub container: Option<String>,
    pub cases: Option<Vec<(Tokens, Tokens)>>,
    pub default_case: Tokens,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Range {
    Full(std::ops::RangeInclusive<usize>),
    To(std::ops::RangeFrom<usize>),
    From(std::ops::RangeTo<usize>),
}

pub type Tokens = Vec<Token>;
pub(crate) type FunctionRef = fn(&mut State, Tokens) -> Result<Token, Report>;
pub type State = HashMap<String, Vec<(Tokens, Tokens)>>;
