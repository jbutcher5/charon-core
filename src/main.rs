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
my_sum <- #n sum
( 3 8 ) my_sum OUTPUT
",
    ));
}
