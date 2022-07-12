use crate::models::{State, Token, Token::*, WFunc, WTokens};
use crate::utils::{convert, encode_string, type_of, Utils};
use crate::evaluator::WEval;
use charon_ariadne::Report;
use itertools::Itertools;
use phf::phf_map;

fn type_of_container(_state: &State, par: WTokens) -> Result<WTokens, Report> {
    Ok(vec![encode_string(&type_of(&par[0]))])
}

fn sum(_state: &State, par: WTokens) -> Result<WTokens, Report> {
    let x = if let Group(x) | List(x) = &par[0] {
        x
    } else {
        unimplemented!()
    };

    Ok(vec![Value(x.as_nums().iter().sum())])
}

fn add(_state: &State, par: WTokens) -> Result<WTokens, Report> {
    let (x, y) = if let [Value(x), Value(y)] = par.as_slice() {
        (x, y)
    } else {
        unimplemented!()
    };

    Ok(vec![Token::Value(x + y)])
}

fn sub(_state: &State, par: WTokens) -> Result<WTokens, Report> {
    let (x, y) = if let [Value(x), Value(y)] = par.as_slice() {
        (x, y)
    } else {
        unimplemented!()
    };

    Ok(vec![Value(y - x)])
}

fn mul(_state: &State, par: WTokens) -> Result<WTokens, Report> {
    let (x, y) = if let [Value(x), Value(y)] = par.as_slice() {
        (x, y)
    } else {
        unimplemented!()
    };

    Ok(vec![Value(x * y)])
}

fn div(_state: &State, par: WTokens) -> Result<WTokens, Report> {
    let (x, y) = if let [Value(x), Value(y)] = par.as_slice() {
        (x, y)
    } else {
        unimplemented!()
    };

    Ok(vec![Value(y / x)])
}

fn modulo(_state: &State, par: WTokens) -> Result<WTokens, Report> {
    let (x, y) = if let [Value(x), Value(y)] = par.as_slice() {
        (x, y)
    } else {
        unimplemented!()
    };

    Ok(vec![Value(y % x)])
}

fn len(_state: &State, par: WTokens) -> Result<WTokens, Report> {
    let x = if let Group(x) | List(x) = &par[0] {
        x
    } else {
        unimplemented!()
    };

    Ok(vec![Value(x.len() as f64)])
}

fn reverse(_state: &State, par: WTokens) -> Result<WTokens, Report> {
    let x = if let Group(x) | List(x) = &par[0] {
        x
    } else {
        unimplemented!()
    };

    Ok(vec![Group(x.iter().rev().cloned().collect::<Vec<_>>())])
}

fn output(_state: &State, par: WTokens) -> Result<WTokens, Report> {
    println!("{}", convert(&par[0]));
    Ok(vec![])
}

fn eq(_state: &State, par: WTokens) -> Result<WTokens, Report> {
    let parameters: (Token, Token) = par.iter().cloned().collect_tuple().unwrap();

    let equal = match parameters {
        (Value(x), Value(y)) => x == y,
        (Function(x), Function(y)) | (FunctionLiteral(x), FunctionLiteral(y)) => x == y,
        (Container(x), Container(y))
        | (Atom(x), Atom(y))
        | (ContainerLiteral(x), ContainerLiteral(y)) => x == y,
        (Parameter(x), Parameter(y)) => x == y,
        _ => unimplemented!(),
    };

    let result = Value(match equal {
        true => 1.0,
        false => 0.0,
    });

    Ok(vec![result])
}

fn or(_state: &State, par: WTokens) -> Result<WTokens, Report> {
    let (x, y) = if let [Value(x), Value(y)] = par.as_slice() {
        (x, y)
    } else {
        unimplemented!()
    };

    let result = if *x == 1.0 || *y == 1.0 {
        Value(1.0)
    } else {
        Value(0.0)
    };

    Ok(vec![result])
}

fn not(_state: &State, par: WTokens) -> Result<WTokens, Report> {
    let x = if let [Value(x)] = par.as_slice() {
        x
    } else {
        unimplemented!()
    };

    let result = if *x != 0.0 {
        Value(0.0)
    } else {
        Value(1.0)
    };

    Ok(vec![result])
}

fn and(_state: &State, par: WTokens) -> Result<WTokens, Report> {
    let (x, y) = if let [Value(x), Value(y)] = par.as_slice() {
        (x, y)
    } else {
        unimplemented!()
    };

    let result = if *x == 1.0 && *y == 1.0 {
        Value(1.0)
    } else {
        Value(0.0)
    };

    Ok(vec![result])
}

fn greater(_state: &State, par: WTokens) -> Result<WTokens, Report> {
    let (x, y) = if let [Value(x), Value(y)] = par.as_slice() {
        (x, y)
    } else {
        unimplemented!()
    };

    let result = if *x > *y {
        Value(1.0)
    } else {
        Value(0.0)
    };

    Ok(vec![result])
}

fn less(_state: &State, par: WTokens) -> Result<WTokens, Report> {
    let (x, y) = if let [Value(x), Value(y)] = par.as_slice() {
        (x, y)
    } else {
        unimplemented!()
    };

    let result = if x < y { Value(1.0) } else { Value(0.0) };

    Ok(vec![result])
}

fn release(_state: &State, par: WTokens) -> Result<WTokens, Report> {
    let x = if let Group(x) = &par[0] {
        x
    } else {
        unimplemented!()
    };
    Ok(x.clone())
}

fn axe(_state: &State, _: WTokens) -> Result<WTokens, Report> {
    Ok(vec![])
}

fn call(_state: &State, par: WTokens) -> Result<WTokens, Report> {
    Ok(vec![match &par[0] {
        ContainerLiteral(x) => Container(x.to_string()),
        FunctionLiteral(x) => Function(x.to_string()),
        _ => unimplemented!(),
    }])
}

fn map(_state: &State, par: WTokens) -> Result<WTokens, Report> {
    let deref = &call(_state, vec![par[0].clone()])?[0];

    let elements: Vec<Token> = if let Group(x) | List(x) = &par[1] {
        x.to_vec()
    } else {
        unimplemented!()
    };

    let mut result = vec![];
    for element in elements {
        result = [result, _state.eval(vec![element, deref.clone()])?].concat();
    }

    Ok(vec![List(result)])
}

pub static COMPLEX_TYPES: phf::Map<&'static str, &[&'static str]> = phf_map! {
    "Literal" => &["FunctionLiteral", "ContainerLiteral"],
    "Iterable" => &["Group", "List"],
};

pub static FUNCTIONS: phf::Map<&'static str, (WFunc, &[&'static str])> = phf_map! {
    "type" => (type_of_container, &["Any"]),
    "sum" => (sum, &["Iterable"]),
    "add" => (add, &["Value", "Value"]),
    "sub" => (sub, &["Value", "Value"]),
    "mul" => (mul, &["Value", "Value"]),
    "div" => (div, &["Value", "Value"]),
    "mod" => (modulo, &["Value", "Value"]),
    "+" => (add, &["Value", "Value"]),
    "-" => (sub, &["Value", "Value"]),
    "*" => (mul, &["Value", "Value"]),
    "/" => (div, &["Value", "Value"]),
    "%" => (modulo, &["Value", "Value"]),
    ">" => (greater, &["Value", "Value"]),
    "<" => (less, &["Value", "Value"]),
    "||" => (or, &["Value", "Value"]),
    "or" => (or, &["Value", "Value"]),
    "&&" => (and, &["Value", "Value"]),
    "and" => (and, &["Value", "Value"]),
    "not" => (not, &["Value"]),
    "len" => (len, &["Iterable"]),
    "reverse" => (reverse, &["Iterable"]),
    "OUTPUT" => (output, &["Any"]),
    "=" => (eq, &["Any", "Any"]),
    "eq" => (eq, &["Any", "Any"]),
    "release" => (release, &[]),
    "axe" => (axe, &["Any"]),
    "swap" => (|_, par| Ok(par), &["Any", "Any"]),
    "call" => (call, &["Literal"]),
    "map" => (map, &["Literal", "Iterable"])
};
