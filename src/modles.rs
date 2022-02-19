#[derive(Debug, Clone)]
pub enum Token {
    Value(f64),
    Function(fn(WCode) -> WCode),
    FunctionLiteral(fn(WCode) -> WCode),
    Container(String),
    Parameter(FunctionParameter),
    Atom(String),
    Special(String),
}

#[derive(Debug, Clone)]
pub enum FunctionParameter {
    Exact(usize),
    Remaining,
}

#[derive(Debug, Clone)]
pub struct WSection {
    pub container: Option<String>,
    pub code: WCode,
}

#[derive(Debug, Clone)]
pub enum WFuncVariant {
    Container(String),
    Function(WFunc)
}

pub type WCode = Vec<Token>;
pub type WFunc = fn(WCode) -> WCode;
