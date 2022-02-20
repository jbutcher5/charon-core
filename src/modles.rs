#[derive(Debug, Clone)]
pub enum Token {
    Value(f64),
    Function(fn(WTokens) -> WTokens),
    FunctionLiteral(fn(WTokens) -> WTokens),
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
    pub code: WTokens,
}

#[derive(Debug, Clone)]
pub enum WFuncVariant {
    Container(String),
    Function(WFunc)
}

pub type WTokens = Vec<Token>;
pub type WFunc = fn(WTokens) -> WTokens;
