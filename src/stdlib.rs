use crate::models::State;
use crate::models::{Range, Token, Token::*, WFunc, WTokens};
use crate::utils::{as_wcode, Utils};
use ariadne::Report;
use itertools::Itertools;
use phf::phf_map;

fn sum(_state: &State, data: WTokens) -> Result<WTokens, Vec<Report>> {
    let nums = data.as_nums();
    Ok(as_wcode(vec![nums.iter().sum()]))
}

fn add(_state: &State, mut data: WTokens) -> Result<WTokens, Vec<Report>> {
    let x = data.get_par(2);
    data.push(Value(
        x.iter()
            .map(|value| match value {
                Value(content) => content,
                _ => panic!("Incorrect type found. Found {:?} but expected Value", data),
            })
            .sum(),
    ));
    Ok(data)
}

fn sub(_state: &State, mut data: WTokens) -> Result<WTokens, Vec<Report>> {
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
    Ok(data)
}

fn mul(_state: &State, mut data: WTokens) -> Result<WTokens, Vec<Report>> {
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
    Ok(data)
}

fn div(_state: &State, mut data: WTokens) -> Result<WTokens, Vec<Report>> {
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
    Ok(data)
}

fn modulo(_state: &State, mut data: WTokens) -> Result<WTokens, Vec<Report>> {
    let x = data.get_par(2);
    let y = x
        .iter()
        .map(|value| match value {
            Value(content) => content,
            _ => panic!("Incorrect type found. Found {:?} but expected Value", data),
        })
        .collect::<Vec<_>>();
    let result = y[1] % y[0];
    data.push(Value(result));
    Ok(data)
}

fn len(_state: &State, mut data: WTokens) -> Result<WTokens, Vec<Report>> {
    let x = &data.get_par(1)[0];

    let length = if let Group(contents) = x {
        contents.len() as f64
    } else {
        1.0
    };
    Ok(vec![Value(length)])
}

fn reverse(_state: &State, mut data: WTokens) -> Result<WTokens, Vec<Report>> {
    let x = &data.get_par(1)[0];

    let y = match x {
        Group(content) => content,
        _ => panic!("Incorrect type found. Found {:?} but expected Group", x),
    };

    Ok(y.iter().rev().cloned().collect::<Vec<_>>())
}

fn elem(_state: &State, mut data: WTokens) -> Result<WTokens, Vec<Report>> {
    let x = &data.get_par(1)[0];
    data.reverse();

    let mut extracted = match x {
        Token::Range(Range::Full(range)) => data.splice(range.clone(), vec![]).collect::<WTokens>(),
        _ => panic!("Incorrect type found. Found {:?} but expected a Range", x),
    };

    extracted.reverse();
    data.reverse();
    data.append(&mut extracted);
    Ok(data)
}

fn copy_elem(_state: &State, mut data: WTokens) -> Result<WTokens, Vec<Report>> {
    let x = &data.get_par(1)[0];
    let mut data_clone: WTokens = data.clone();
    data_clone.reverse();

    let mut extracted = match x {
        Token::Range(Range::Full(range)) => data_clone
            .splice(range.clone(), vec![])
            .collect::<WTokens>(),
        _ => panic!("Incorrect type found. Found {:?} but expected Range", x),
    };

    extracted.reverse();
    data_clone.reverse();
    data.append(&mut extracted);
    Ok(data)
}

fn output(_state: &State, mut data: WTokens) -> Result<WTokens, Vec<Report>> {
    let mut x = data.get_par(1);
    println!("{}", x.literal());
    data.append(&mut x);
    Ok(data)
}

fn eq(_state: &State, mut data: WTokens) -> Result<WTokens, Vec<Report>> {
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
    Ok(data)
}

fn or(_state: &State, mut data: WTokens) -> Result<WTokens, Vec<Report>> {
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
    Ok(data)
}

fn not(_state: &State, mut data: WTokens) -> Result<WTokens, Vec<Report>> {
    let par = &data.get_par(1)[0];

    let token = Value(match par {
        Value(x) => {
            if *x == 0.0 {
                1.0
            } else {
                0.0
            }
        }
        _ => panic!("Incorrect type found. Found {:?} but expected Value", par),
    });

    data.push(token);
    Ok(data)
}

fn and(_state: &State, mut data: WTokens) -> Result<WTokens, Vec<Report>> {
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
    Ok(data)
}

fn greater(_state: &State, mut data: WTokens) -> Result<WTokens, Vec<Report>> {
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
    Ok(data)
}

fn less(_state: &State, mut data: WTokens) -> Result<WTokens, Vec<Report>> {
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
    Ok(data)
}

fn if_else(_state: &State, mut data: WTokens) -> Result<WTokens, Vec<Report>> {
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
    Ok(data)
}

fn expand(_state: &State, mut data: WTokens) -> Result<WTokens, Vec<Report>> {
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

    Ok(data)
}

fn release(_state: &State, mut data: WTokens) -> Result<WTokens, Vec<Report>> {
    let par_arr = data.get_par(1);
    let parameters = &par_arr[0];

    match parameters {
        Group(group) => data.append(&mut group.clone()),
        _ => panic!("Incorrect type found. Found {:?} but expected Group", data),
    };

    Ok(data)
}

fn axe(_state: &State, data: WTokens) -> Result<WTokens, Vec<Report>> {
    Ok(data[..data.len() - 1].to_vec())
}

fn bundle(_state: &State, data: WTokens) -> Result<WTokens, Vec<Report>> {
    Ok(vec![Group(data)])
}

pub static FUNCTIONS: phf::Map<&'static str, WFunc> = phf_map! {
    "sum" => sum,
    "add" => add,
    "sub" => sub,
    "mul" => mul,
    "div" => div,
    "mod" => modulo,
    "+" => add,
    "-" => sub,
    "*" => mul,
    "/" => div,
    "%" => modulo,
    ">" => greater,
    "<" => less,
    "||" => or,
    "or" => or,
    "&&" => and,
    "and" => and,
    "not" => not,
    "len" => len,
    "reverse" => reverse,
    "elem" => elem,
    "copy_elem" => copy_elem,
    "OUTPUT" => output,
    "eq" => eq,
    "if" => if_else,
    "expand" => expand,
    "release" => release,
    "axe" => axe,
    "bundle" => bundle
};
