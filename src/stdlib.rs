use crate::evaluator::WEval;
use crate::models::{State, Token, Token::*, WFunc, WTokens};
use crate::utils::{convert, encode_string, type_of, Utils};
use charon_ariadne::Report;
use itertools::Itertools;
use phf::phf_map;

fn type_of_container(_state: &mut State, par: WTokens) -> Result<WTokens, Report> {
    Ok(vec![encode_string(&type_of(&par[0]))])
}

fn sum(_state: &mut State, par: WTokens) -> Result<WTokens, Report> {
    let x = if let Group(x) | List(x) = &par[0] {
        x
    } else {
        unimplemented!()
    };

    Ok(vec![Value(x.as_nums().iter().sum())])
}

fn add(_state: &mut State, par: WTokens) -> Result<WTokens, Report> {
    let (x, y) = if let [Value(x), Value(y)] = par.as_slice() {
        (x, y)
    } else {
        unimplemented!()
    };

    Ok(vec![Token::Value(x + y)])
}

fn sub(_state: &mut State, par: WTokens) -> Result<WTokens, Report> {
    let (x, y) = if let [Value(x), Value(y)] = par.as_slice() {
        (x, y)
    } else {
        unimplemented!()
    };

    Ok(vec![Value(y - x)])
}

fn mul(_state: &mut State, par: WTokens) -> Result<WTokens, Report> {
    let (x, y) = if let [Value(x), Value(y)] = par.as_slice() {
        (x, y)
    } else {
        unimplemented!()
    };

    Ok(vec![Value(x * y)])
}

fn div(_state: &mut State, par: WTokens) -> Result<WTokens, Report> {
    let (x, y) = if let [Value(x), Value(y)] = par.as_slice() {
        (x, y)
    } else {
        unimplemented!()
    };

    Ok(vec![Value(y / x)])
}

fn modulo(_state: &mut State, par: WTokens) -> Result<WTokens, Report> {
    let (x, y) = if let [Value(x), Value(y)] = par.as_slice() {
        (x, y)
    } else {
        unimplemented!()
    };

    Ok(vec![Value(y % x)])
}

fn len(_state: &mut State, par: WTokens) -> Result<WTokens, Report> {
    let x = if let Group(x) | List(x) = &par[0] {
        x
    } else {
        unimplemented!()
    };

    Ok(vec![Value(x.len() as f64)])
}

fn reverse(_state: &mut State, par: WTokens) -> Result<WTokens, Report> {
    if let Group(x) = &par[0] {
        Ok(vec![Group(x.iter().rev().cloned().collect::<Vec<_>>())])
    } else if let List(x) = &par[0] {
        Ok(vec![List(x.iter().rev().cloned().collect::<Vec<_>>())])
    } else {
        unimplemented!()
    }
}

fn output(_state: &mut State, par: WTokens) -> Result<WTokens, Report> {
    println!("{}", convert(&par[0]));
    Ok(vec![])
}

fn eq(_state: &mut State, par: WTokens) -> Result<WTokens, Report> {
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

fn or(_state: &mut State, par: WTokens) -> Result<WTokens, Report> {
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

fn not(_state: &mut State, par: WTokens) -> Result<WTokens, Report> {
    let x = if let [Value(x)] = par.as_slice() {
        x
    } else {
        unimplemented!()
    };

    let result = if *x != 0.0 { Value(0.0) } else { Value(1.0) };

    Ok(vec![result])
}

fn and(_state: &mut State, par: WTokens) -> Result<WTokens, Report> {
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

fn greater(_state: &mut State, par: WTokens) -> Result<WTokens, Report> {
    let (x, y) = if let [Value(x), Value(y)] = par.as_slice() {
        (x, y)
    } else {
        unimplemented!()
    };

    let result = if *x > *y { Value(1.0) } else { Value(0.0) };

    Ok(vec![result])
}

fn less(_state: &mut State, par: WTokens) -> Result<WTokens, Report> {
    let (x, y) = if let [Value(x), Value(y)] = par.as_slice() {
        (x, y)
    } else {
        unimplemented!()
    };

    let result = if x < y { Value(1.0) } else { Value(0.0) };

    Ok(vec![result])
}

fn release(_state: &mut State, par: WTokens) -> Result<WTokens, Report> {
    let x = if let Group(x) = &par[0] {
        x
    } else {
        unimplemented!()
    };
    Ok(x.clone())
}

fn axe(_state: &mut State, _: WTokens) -> Result<WTokens, Report> {
    Ok(vec![])
}

fn call(_state: &mut State, par: WTokens) -> Result<WTokens, Report> {
    Ok(vec![match &par[0] {
        ContainerLiteral(x) => Container(x.to_string()),
        FunctionLiteral(x) => Function(x.to_string()),
        Lambda(lambda) => ActiveLambda(lambda.to_vec()),
        _ => unimplemented!(),
    }])
}

fn map(_state: &mut State, par: WTokens) -> Result<WTokens, Report> {
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

fn foldr(_state: &mut State, par: WTokens) -> Result<WTokens, Report> {
    let mut acc: Token = par[0].clone();
    let arr = if let Group(x) | List(x) = &par[2] {
        x.to_vec()
    } else {
        unimplemented!()
    };
    let func = &call(_state, vec![par[1].clone()])?[0];

    for element in arr {
        acc = _state.eval(vec![acc.clone(), element, func.clone()])?[0].clone();
    }

    Ok(vec![acc.clone()])
}

fn foldl(_state: &mut State, mut par: WTokens) -> Result<WTokens, Report> {
    let mut reversed = reverse(_state, vec![par.pop().unwrap()])?;
    par.append(&mut reversed);
    foldr(_state, par)
}

fn lambda(_state: &mut State, par: WTokens) -> Result<WTokens, Report> {
    if let List(x) = &par[0] {
        Ok(vec![Lambda(x.to_vec())])
    } else {
        unimplemented!()
    }
}

fn head(_state: &mut State, par: WTokens) -> Result<WTokens, Report> {
    if let Group(x) | List(x) = &par[0] {
        if let Some(first) = x.get(0) {
            Ok(vec![first.clone()])
        } else {
            Ok(vec![])
        }
    } else {
        unimplemented!()
    }
}

fn tail(_state: &mut State, par: WTokens) -> Result<WTokens, Report> {
    match par[0].clone() {
        Group(mut x) => {
            x.remove(0);
            Ok(vec![Group(x.clone())])
        }
        List(mut x) => {
            x.remove(0);
            Ok(vec![List(x.clone())])
        }
        _ => unimplemented!(),
    }
}

pub static COMPLEX_TYPES: phf::Map<&'static str, &[&'static str]> = phf_map! {
    "Literal" => &["Lambda", "FunctionLiteral", "ContainerLiteral"],
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
    "greater" => (greater, &["Value", "Value"]),
    "less" => (less, &["Value", "Value"]),
    "or" => (or, &["Value", "Value"]),
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
    "map" => (map, &["Literal", "Iterable"]),
    "foldr" => (foldr, &["Any", "Literal", "Iterable"]),
    "foldl" => (foldl, &["Any", "Literal", "Iterable"]),
    "lambda" => (lambda, &["List"]),
    "head" => (head, &["Iterable"]),
    "tail" => (tail, &["Iterable"])
};
