mod evaluator;
mod lexer;
mod models;
mod stdlib;
mod utils;
mod preprocessor;

use crate::evaluator::WEval;
use crate::lexer::lexer;
use crate::models::{State, WTokens};

pub fn eval(code: &str) -> Vec<WTokens> {
    let mut state = State::new();
    state.wsection_eval(lexer(code))
}
