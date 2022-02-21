use crate::models::{FunctionParameter, Token, WFunc, WTokens};
use crate::utils::{as_nums, as_wcode};
use phf::phf_map;

fn sum(data: WTokens) -> WTokens {
    let nums = as_nums(data);
    as_wcode(vec![nums.iter().sum()])
}

fn add(mut data: WTokens) -> WTokens {
    let x = as_nums(vec![data.pop().unwrap(), data.pop().unwrap()]);
    data.push(Token::Value(x.iter().sum()));
    data
}

fn sub(mut data: WTokens) -> WTokens {
    let x = as_nums(vec![data.pop().unwrap(), data.pop().unwrap()]);
    let result = x[1] - x[0];
    data.push(Token::Value(result));
    data
}

fn mul(mut data: WTokens) -> WTokens {
    let x = as_nums(vec![data.pop().unwrap(), data.pop().unwrap()]);
    let result = x[1] * x[0];
    data.push(Token::Value(result));
    data
}

fn div(mut data: WTokens) -> WTokens {
    let x = as_nums(vec![data.pop().unwrap(), data.pop().unwrap()]);
    let result = x[1] / x[0];
    data.push(Token::Value(result));
    data
}

fn len(data: WTokens) -> WTokens {
    let length = data.len() as f64;
    vec![Token::Value(length)]
}

fn output(data: WTokens) -> WTokens {
    let result = data
        .clone()
        .iter()
        .fold(String::new(), |acc, token| match token {
            Token::Value(x) => format!("{} {}", acc, x),
            Token::Atom(x) | Token::Special(x) | Token::Container(x) => format!("{} {}", acc, x),
            Token::Function(func) | Token::FunctionLiteral(func) => format!("{} {:?}", acc, func),
            Token::Parameter(FunctionParameter::Exact(index)) => format!("{} #{}", acc, index),
            Token::Parameter(FunctionParameter::Remaining) => format!("{} #n", acc),
        });

    println!("{}", result);
    data
}

fn eq(mut data: WTokens) -> WTokens {
    let parameters = (data.pop().unwrap(), data.pop().unwrap());

    let result = match parameters {
        (Token::Value(x), Token::Value(y)) => x == y,
        (Token::Function(x), Token::Function(y))
        | (Token::FunctionLiteral(x), Token::FunctionLiteral(y)) => x == y,
        (Token::Container(x), Token::Container(y)) | (Token::Atom(x), Token::Atom(y)) => x == y,
        (Token::Parameter(x), Token::Parameter(y)) => x == y,
        _ => panic!("Incorrect tokens"),
    };

    let token = Token::Value(match result {
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
