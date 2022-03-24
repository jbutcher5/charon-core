mod evaluator;
mod lexer;
mod stdlib;
mod utils;
mod parser;
pub mod models;

use crate::evaluator::WEval;
use crate::models::{State, WTokens};

pub fn eval(code: &str) -> Vec<WTokens> {
    let mut state = State::new();
    state.wsection_eval(parser::parser(code))
}
