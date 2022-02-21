mod evaluator;
mod lexer;
mod models;
mod stdlib;
mod utils;

use crate::evaluator::wsection_eval;
use crate::lexer::lexer;

fn main() {
    wsection_eval(lexer(
        "
x <- 5
y <- 4
y x 1 12 eq if-else OUTPUT
",
    ));
}
