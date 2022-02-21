use crate::models::{FunctionParameter, Token::*, WFunc, WTokens};
use crate::utils::{as_nums, as_wcode};
use phf::phf_map;

fn sum(data: WTokens) -> WTokens {
    let nums = as_nums(data);
    as_wcode(vec![nums.iter().sum()])
}

fn add(mut data: WTokens) -> WTokens {
    let x = as_nums(vec![data.pop().unwrap(), data.pop().unwrap()]);
    data.push(Value(x.iter().sum()));
    data
}

fn sub(mut data: WTokens) -> WTokens {
    let x = as_nums(vec![data.pop().unwrap(), data.pop().unwrap()]);
    let result = x[1] - x[0];
    data.push(Value(result));
    data
}

fn mul(mut data: WTokens) -> WTokens {
    let x = as_nums(vec![data.pop().unwrap(), data.pop().unwrap()]);
    let result = x[1] * x[0];
    data.push(Value(result));
    data
}

fn div(mut data: WTokens) -> WTokens {
    let x = as_nums(vec![data.pop().unwrap(), data.pop().unwrap()]);
    let result = x[1] / x[0];
    data.push(Value(result));
    data
}

fn len(data: WTokens) -> WTokens {
    let length = data.len() as f64;
    vec![Value(length)]
}

fn output(data: WTokens) -> WTokens {
    let result = data
        .clone()
        .iter()
        .fold(String::new(), |acc, token| match token {
            Value(x) => format!("{} {}", acc, x),
            Atom(x) | Special(x) | Container(x) | ContainerLiteral(x) => format!("{} {}", acc, x),
            Function(func) | FunctionLiteral(func) => format!("{} {:?}", acc, func),
            Parameter(FunctionParameter::Exact(index)) => format!("{} #{}", acc, index),
            Parameter(FunctionParameter::Remaining) => format!("{} #n", acc),
        });

    println!("{}", result);
    data
}

fn eq(mut data: WTokens) -> WTokens {
    let parameters = (data.pop().unwrap(), data.pop().unwrap());

    let result = match parameters {
        (Value(x), Value(y)) => x == y,
        (Function(x), Function(y)) | (FunctionLiteral(x), FunctionLiteral(y)) => x == y,
        (Container(x), Container(y))
        | (Atom(x), Atom(y))
        | (ContainerLiteral(x), ContainerLiteral(y)) => x == y,
        (Parameter(x), Parameter(y)) => x == y,
        _ => panic!("Incorrect tokens"),
    };

    let token = Value(match result {
        true => 1.0,
        false => 0.0,
    });

    data.push(token);
    data
}

pub static FUNCTIONS: phf::Map<&'static str, WFunc> = phf_map! {
    "sum" => sum,
    "add" => add,
    "sub" => sub,
    "mul" => mul,
    "div" => div,
    "+" => add,
    "-" => sub,
    "*" => mul,
    "/" => div,
    "len" => len,
    "OUTPUT" => output,
    "eq" => eq
};
