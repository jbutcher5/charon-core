use crate::models::{Token, Token::*, WFunc, WTokens};
use crate::utils::{as_wcode, Utils};
use itertools::Itertools;
use phf::phf_map;

fn sum(data: WTokens) -> WTokens {
    let nums = data.as_nums();
    as_wcode(vec![nums.iter().sum()])
}

fn add(mut data: WTokens) -> WTokens {
    let x = data.get_par(2);
    data.push(Value(
        x.iter()
            .map(|value| match value {
                Value(content) => content,
                _ => panic!("Incorrect type found. Found {:?} but expected Value", data),
            })
            .sum(),
    ));
    data
}

fn sub(mut data: WTokens) -> WTokens {
    let x = data.get_par(2);
    let y = x
        .iter()
        .map(|value| match value {
            Value(content) => content,
            _ => panic!("Incorrect type found. Found {:?} but expected Value", data),
        })
        .collect::<Vec<_>>();
    let result = y[1] - y[0];
    data.push(Value(result));
    data
}

fn mul(mut data: WTokens) -> WTokens {
    let x = data.get_par(2);
    let y = x
        .iter()
        .map(|value| match value {
            Value(content) => content,
            _ => panic!("Incorrect type found. Found {:?} but expected Value", data),
        })
        .collect::<Vec<_>>();
    let result = y[1] * y[0];
    data.push(Value(result));
    data
}

fn div(mut data: WTokens) -> WTokens {
    let x = data.get_par(2);
    let y = x
        .iter()
        .map(|value| match value {
            Value(content) => content,
            _ => panic!("Incorrect type found. Found {:?} but expected Value", data),
        })
        .collect::<Vec<_>>();
    let result = y[1] / y[0];
    data.push(Value(result));
    data
}

fn len(data: WTokens) -> WTokens {
    let length = data.len() as f64;
    vec![Value(length)]
}

fn reverse(data: WTokens) -> WTokens {
    data.iter().rev().cloned().collect::<Vec<_>>()
}

fn output(data: WTokens) -> WTokens {
    println!("{}", data.literal());
    data
}

fn eq(mut data: WTokens) -> WTokens {
    let par_arr = data.get_par(2);
    let parameters: (Token, Token) = par_arr.iter().cloned().collect_tuple().unwrap();

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
    let par_arr = data.get_par(2);
    let parameters: (Token, Token) = par_arr.iter().cloned().collect_tuple().unwrap();

    let token = Value(match parameters {
        (Value(x), Value(y)) => match (x != 0.0) || (y != 0.0) {
            true => 1.0,
            _ => 0.0,
        },
        _ => 0.0,
    });

    data.push(token);
    data
}

fn and(mut data: WTokens) -> WTokens {
    let par_arr = data.get_par(2);
    let parameters: (Token, Token) = par_arr.iter().cloned().collect_tuple().unwrap();

    let token = Value(match parameters {
        (Value(x), Value(y)) => match (x != 0.0) && (y != 0.0) {
            true => 1.0,
            _ => 0.0,
        },
        _ => 0.0,
    });

    data.push(token);
    data
}

fn greater(mut data: WTokens) -> WTokens {
    let par_arr = data.get_par(2);
    let parameters: (Token, Token) = par_arr.iter().cloned().collect_tuple().unwrap();

    let token = Value(match parameters {
        (Value(x), Value(y)) => match y > x {
            true => 1.0,
            _ => 0.0,
        },
        _ => 0.0,
    });

    data.push(token);
    data
}

fn less(mut data: WTokens) -> WTokens {
    let par_arr = data.get_par(2);
    let parameters: (Token, Token) = par_arr.iter().cloned().collect_tuple().unwrap();

    let token = Value(match parameters {
        (Value(x), Value(y)) => match y < x {
            true => 1.0,
            _ => 0.0,
        },
        _ => 0.0,
    });

    data.push(token);
    data
}

fn if_else(mut data: WTokens) -> WTokens {
    let par_arr = data.get_par(3);
    let parameters: (Token, Token, Token) = par_arr.iter().cloned().collect_tuple().unwrap();

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

fn expand(mut data: WTokens) -> WTokens {
    let par_arr = data.get_par(2);
    let parameters: (Token, Token) = par_arr.iter().cloned().collect_tuple().unwrap();

    match parameters {
        (Value(n), Group(group)) => {
            for _ in 0..n as usize {
                data.append(&mut group.clone())
            }
        }
        _ => panic!(
            "Incorrect type found. Found {:?} but expected (Value, Group)",
            data
        ),
    }

    data
}

fn release(mut data: WTokens) -> WTokens {
    let par_arr = data.get_par(1);
    let parameters = &par_arr[0];

    match parameters {
        Group(group) => data.append(&mut group.clone()),
        _ => panic!("Incorrect type found. Found {:?} but expected Group", data),
    };

    data
}

fn bundle(data: WTokens) -> WTokens {
    vec![Group(data)]
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
    ">" => greater,
    "<" => less,
    "||" => or,
    "or" => or,
    "&&" => and,
    "and" => and,
    "len" => len,
    "reverse" => reverse,
    "OUTPUT" => output,
    "eq" => eq,
    "if" => if_else,
    "expand" => expand,
    "release" => release,
    "bundle" => bundle
};
