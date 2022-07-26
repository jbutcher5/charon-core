use crate::evaluator::Evaluate;
use crate::utils::{convert, encode_string, type_of, Utils};
use crate::{FunctionRef, State, Token, Token::*, Tokens};
use charon_ariadne::Report;
use itertools::Itertools;
use phf::phf_map;

fn type_of_container(_state: &mut State, par: Tokens) -> Result<Token, Report> {
    Ok(encode_string(&type_of(&par[0])))
}

fn sum(_state: &mut State, par: Tokens) -> Result<Token, Report> {
    let x = if let Group(x) | List(x) = &par[0] {
        x
    } else {
        unimplemented!()
    };

    Ok(Value(x.as_nums().iter().sum()))
}

fn add(_state: &mut State, par: Tokens) -> Result<Token, Report> {
    let (x, y) = if let [Value(x), Value(y)] = par.as_slice() {
        (x, y)
    } else {
        unimplemented!()
    };

    Ok(Token::Value(x + y))
}

fn sub(_state: &mut State, par: Tokens) -> Result<Token, Report> {
    let (x, y) = if let [Value(x), Value(y)] = par.as_slice() {
        (x, y)
    } else {
        unimplemented!()
    };

    Ok(Value(y - x))
}

fn mul(_state: &mut State, par: Tokens) -> Result<Token, Report> {
    let (x, y) = if let [Value(x), Value(y)] = par.as_slice() {
        (x, y)
    } else {
        unimplemented!()
    };

    Ok(Value(x * y))
}

fn div(_state: &mut State, par: Tokens) -> Result<Token, Report> {
    let (x, y) = if let [Value(x), Value(y)] = par.as_slice() {
        (x, y)
    } else {
        unimplemented!()
    };

    Ok(Value(y / x))
}

fn modulo(_state: &mut State, par: Tokens) -> Result<Token, Report> {
    let (x, y) = if let [Value(x), Value(y)] = par.as_slice() {
        (x, y)
    } else {
        unimplemented!()
    };

    Ok(Value(y % x))
}

fn len(_state: &mut State, par: Tokens) -> Result<Token, Report> {
    let x = if let Group(x) | List(x) = &par[0] {
        x
    } else {
        unimplemented!()
    };

    Ok(Value(x.len() as f64))
}

fn reverse(_state: &mut State, par: Tokens) -> Result<Token, Report> {
    if let Group(x) = &par[0] {
        Ok(Group(x.iter().rev().cloned().collect::<Vec<_>>()))
    } else if let List(x) = &par[0] {
        Ok(List(x.iter().rev().cloned().collect::<Vec<_>>()))
    } else {
        unimplemented!()
    }
}

fn output(_state: &mut State, par: Tokens) -> Result<Token, Report> {
    println!("{}", convert(&par[0]));
    Ok(Void)
}

fn eq(_state: &mut State, par: Tokens) -> Result<Token, Report> {
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

    Ok(Value(match equal {
        true => 1.0,
        false => 0.0,
    }))
}

fn or(_state: &mut State, par: Tokens) -> Result<Token, Report> {
    let (x, y) = if let [Value(x), Value(y)] = par.as_slice() {
        (x, y)
    } else {
        unimplemented!()
    };

    Ok(if *x == 1.0 || *y == 1.0 {
        Value(1.0)
    } else {
        Value(0.0)
    })
}

fn not(_state: &mut State, par: Tokens) -> Result<Token, Report> {
    let x = if let [Value(x)] = par.as_slice() {
        x
    } else {
        unimplemented!()
    };

    Ok(if *x != 0.0 { Value(0.0) } else { Value(1.0) })
}

fn and(_state: &mut State, par: Tokens) -> Result<Token, Report> {
    let (x, y) = if let [Value(x), Value(y)] = par.as_slice() {
        (x, y)
    } else {
        unimplemented!()
    };

    Ok(if *x == 1.0 && *y == 1.0 {
        Value(1.0)
    } else {
        Value(0.0)
    })
}

fn greater(_state: &mut State, par: Tokens) -> Result<Token, Report> {
    let (x, y) = if let [Value(x), Value(y)] = par.as_slice() {
        (x, y)
    } else {
        unimplemented!()
    };

    Ok(if *x > *y { Value(1.0) } else { Value(0.0) })
}

fn less(_state: &mut State, par: Tokens) -> Result<Token, Report> {
    let (x, y) = if let [Value(x), Value(y)] = par.as_slice() {
        (x, y)
    } else {
        unimplemented!()
    };

    Ok(if x < y { Value(1.0) } else { Value(0.0) })
}

fn axe(_state: &mut State, _: Tokens) -> Result<Token, Report> {
    Ok(Void)
}

fn call(_state: &mut State, par: Tokens) -> Result<Token, Report> {
    Ok(match &par[0] {
        ContainerLiteral(x) => Container(x.to_string()),
        FunctionLiteral(x) => Function(x.to_string()),
        Lambda(lambda) => ActiveLambda(lambda.to_vec()),
        _ => unimplemented!(),
    })
}

fn map(_state: &mut State, par: Tokens) -> Result<Token, Report> {
    let deref = &call(_state, vec![par[0].clone()])?;

    let elements: Vec<Token> = if let Iterator(x) = &par[1] {
        x.to_vec()
    } else {
        unimplemented!()
    };

    let mut result = vec![];
    for element in elements {
        result = [result, _state.eval(vec![element, deref.clone()])?].concat();
    }

    Ok(Iterator(result))
}

fn foldr(_state: &mut State, par: Tokens) -> Result<Token, Report> {
    let mut acc: Token = par[0].clone();
    let arr = if let Iterator(x) = &par[2] {
        x.to_vec()
    } else {
        unimplemented!()
    };
    let func = &call(_state, vec![par[1].clone()])?;

    for element in arr {
        acc = _state.eval(vec![acc.clone(), element, func.clone()])?[0].clone();
    }

    Ok(acc)
}

fn foldl(_state: &mut State, mut par: Tokens) -> Result<Token, Report> {
    let reversed: Token = if let Iterator(x) = &par[0] {
        Iterator(x.iter().cloned().rev().collect())
    } else {
        unimplemented!()
    };
    par.push(reversed);
    foldr(_state, par)
}

fn iter(_state: &mut State, par: Tokens) -> Result<Token, Report> {
    Ok(if let Group(x) | List(x) = &par[0] {
        Iterator(x.to_vec())
    } else {
        unimplemented!()
    })
}

fn collect_group(_state: &mut State, par: Tokens) -> Result<Token, Report> {
    let arr = if let Iterator(x) = &par[0] {
        x.to_vec()
    } else {
        unimplemented!()
    };

    Ok(Group(arr))
}

fn collect_list(_state: &mut State, par: Tokens) -> Result<Token, Report> {
    let arr = if let Iterator(x) = &par[0] {
        x.to_vec()
    } else {
        unimplemented!()
    };

    Ok(List(arr))
}

fn lambda(_state: &mut State, par: Tokens) -> Result<Token, Report> {
    if let List(x) = &par[0] {
        Ok(Lambda(x.to_vec()))
    } else {
        unimplemented!()
    }
}

fn head(_state: &mut State, par: Tokens) -> Result<Token, Report> {
    if let Group(x) | List(x) = &par[0] {
        if let Some(first) = x.get(0) {
            Ok(first.clone())
        } else {
            Ok(Null)
        }
    } else {
        unimplemented!()
    }
}

fn tail(_state: &mut State, par: Tokens) -> Result<Token, Report> {
    match par[0].clone() {
        Group(mut x) => {
            x.remove(0);
            Ok(Group(x.clone()))
        }
        List(mut x) => {
            x.remove(0);
            Ok(List(x.clone()))
        }
        _ => unimplemented!(),
    }
}

pub static COMPLEX_TYPES: phf::Map<&'static str, &[&'static str]> = phf_map! {
    "Literal" => &["Lambda", "FunctionLiteral", "ContainerLiteral"],
    "Iterable" => &["Group", "List"],
};

pub static FUNCTIONS: phf::Map<&'static str, (FunctionRef, &[&'static str])> = phf_map! {
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
    "axe" => (axe, &["Any"]),
    "swap" => (|_, par| Ok(par[0].clone()), &["Any", "Any"]),
    "call" => (call, &["Literal"]),
    "map" => (map, &["Literal", "Iterator"]),
    "foldr" => (foldr, &["Any", "Literal", "Iterator"]),
    "foldl" => (foldl, &["Any", "Literal", "Iterator"]),
    "iter" => (iter, &["Iterable"]),
    "collect_group" => (collect_group, &["Iterator"]),
    "collect_list" => (collect_list, &["Iterator"]),
    "lambda" => (lambda, &["List"]),
    "head" => (head, &["Iterable"]),
    "tail" => (tail, &["Iterable"])
};
