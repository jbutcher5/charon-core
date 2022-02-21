mod models;
mod stdlib;
mod utils;
mod lexer;
mod evaluator;

use crate::lexer::lexer;
use crate::evaluator::wsection_eval;

fn main() {
    wsection_eval(lexer(
"
arr <- 1 4 6 2 5 1
mean <- ( #n sum ) ( #n len ) div
arr mean OUTPUT
"
));
}
