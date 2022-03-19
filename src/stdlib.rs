use crate::models::{Token::*, WFunc, WTokens};
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

fn reverse(mut data: WTokens) -> WTokens {
    data.reverse();
    return data;
}

fn output(data: WTokens) -> WTokens {
    let result = data
        .clone()
        .iter()
        .fold(String::new(), |acc, token| -> String {
            match token {
                Value(x) => format!("{} {}", acc, x),
                Atom(x) | Special(x) | Container(x) | ContainerLiteral(x) => {
                    format!("{} {}", acc, x)
                }
                _ => format!("{} {:?}", acc, token),
            }
        });

    println!("{}", result.trim());
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

fn or(mut data: WTokens) -> WTokens {

    let parameters = (data.pop().unwrap(), data.pop().unwrap());

    let token = Value(match parameters {
        (Value(x), Value(y)) => match (x != 0.0) || (y != 0.0) {
            true => 1.0,
            _ => 0.0
        },
        _ => 0.0
    });

    data.push(token);
    data
}

fn if_else(mut data: WTokens) -> WTokens {
    let parameters = (
        data.pop().unwrap(),
        data.pop().unwrap(),
        data.pop().unwrap(),
    );

    let selected = match parameters.0 {
        Value(x) => {
            if x != 0.0 {
                parameters.1
            } else {
                parameters.2
            }
        }

        _ => parameters.2,
    };

    let mut convered = match selected {
        Group(x) => x,
        ContainerLiteral(x) => vec![Container(x)],
        FunctionLiteral(x) => vec![Function(x)],
        _ => vec![selected],
    };

    data.append(&mut convered);
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
    "||" => or,
    "len" => len,
    "reverse" => reverse,
    "OUTPUT" => output,
    "eq" => eq,
    "or" => or,
    "if-else" => if_else
};
