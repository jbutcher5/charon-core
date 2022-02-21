mod modles;
mod stdlib;
mod utils;
mod lexer;
mod evaluator;

use crate::lexer::lexer;
use crate::evaluator::wsection_eval;

fn main() {
    wsection_eval(lexer(
"
1.1 1 eq OUTPUT
"
));
}
