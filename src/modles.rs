#[derive(Debug, Clone)]
pub enum Token {
    Value(f64),
    Function(fn(WCode) -> WCode),
    FunctionLiteral(fn(WCode) -> WCode),
    Atom(String),
    Special(String),
}

pub type WCode = Vec<Token>;
pub type WFunc = fn(WCode) -> WCode;
