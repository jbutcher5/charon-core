mod evaluator;
mod lexer;
mod models;
mod stdlib;
mod utils;

use crate::evaluator::wsection_eval;
use crate::lexer::lexer;
use crate::models::WTokens;

pub fn eval(code: &str) -> Vec<WTokens> {
    wsection_eval(lexer(code))
}
