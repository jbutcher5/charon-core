use crate::models::State;
use crate::models::{Range, Token, Token::*, WFunc, WTokens};
use crate::utils::{as_wcode, type_of, Utils};
use ariadne::{Color, Label, Report, ReportKind, Source};
use itertools::Itertools;
use phf::phf_map;

fn type_of_container(_state: &State, mut data: WTokens) -> Result<WTokens, Vec<Report>> {
    let par = &data.get_par(1)[0];
    let type_string = Group(type_of(par).chars().map(Char).collect::<_>());

    data.push(type_string);
    Ok(data)
}

fn sum(_state: &State, data: WTokens) -> Result<WTokens, Vec<Report>> {
    let nums = data.as_nums();
    Ok(as_wcode(vec![nums.iter().sum()]))
}

fn add(_state: &State, mut data: WTokens) -> Result<WTokens, Vec<Report>> {
    let par_arr = data.get_par(2);
    let full = [
        data.clone(),
        par_arr.iter().cloned().rev().collect::<Vec<_>>(),
        vec![Token::Function("add".to_string())],
    ]
    .concat();
    let parameters: (Token, Token) = par_arr.clone().iter().cloned().collect_tuple().unwrap();

    let result = Value(match parameters {
        (Value(x), Value(y)) => x + y,
        _ => {
            let literal = full.literal_enumerate();

            return Err(vec![Report::build(ReportKind::Error, (), 0)
                .with_message("Invalid Operation")
                .with_label(
                    Label::new(literal.1[literal.1.len() - 2].clone())
                        .with_message(format!("This has the type of {:?}", type_of(&par_arr[1])))
                        .with_color(Color::Red),
                )
                .with_label(
                    Label::new(literal.1[literal.1.len() - 3].clone())
                        .with_message(format!("This has the type of {:?}", type_of(&par_arr[0])))
                        .with_color(Color::Red),
                )
                .with_source(Source::from(literal.0))
                .finish()]);
        }
    });

    data.push(result);
    Ok(data)
}

fn sub(_state: &State, mut data: WTokens) -> Result<WTokens, Vec<Report>> {
    let par_arr = data.get_par(2);
    let full = [
        data.clone(),
        par_arr.iter().cloned().rev().collect::<Vec<_>>(),
        vec![Token::Function("sub".to_string())],
    ]
    .concat();
    let parameters: (Token, Token) = par_arr.clone().iter().cloned().collect_tuple().unwrap();

    let result = Value(match parameters {
        (Value(x), Value(y)) => y - x,
        _ => {
            let literal = full.literal_enumerate();

            return Err(vec![Report::build(ReportKind::Error, (), 0)
                .with_message("Invalid Operation")
                .with_label(
                    Label::new(literal.1[literal.1.len() - 2].clone())
                        .with_message(format!("This has the type of {:?}", type_of(&par_arr[1])))
                        .with_color(Color::Red),
                )
                .with_label(
                    Label::new(literal.1[literal.1.len() - 3].clone())
                        .with_message(format!("This has the type of {:?}", type_of(&par_arr[0])))
                        .with_color(Color::Red),
                )
                .with_source(Source::from(literal.0))
                .finish()]);
        }
    });

    data.push(result);
    Ok(data)
}

fn mul(_state: &State, mut data: WTokens) -> Result<WTokens, Vec<Report>> {
    let par_arr = data.get_par(2);
    let full = [
        data.clone(),
        par_arr.iter().cloned().rev().collect::<Vec<_>>(),
        vec![Token::Function("mul".to_string())],
    ]
    .concat();
    let parameters: (Token, Token) = par_arr.clone().iter().cloned().collect_tuple().unwrap();

    let result = Value(match parameters {
        (Value(x), Value(y)) => x * y,
        _ => {
            let literal = full.literal_enumerate();

            return Err(vec![Report::build(ReportKind::Error, (), 0)
                .with_message("Invalid Operation")
                .with_label(
                    Label::new(literal.1[literal.1.len() - 2].clone())
                        .with_message(format!("This has the type of {:?}", type_of(&par_arr[1])))
                        .with_color(Color::Red),
                )
                .with_label(
                    Label::new(literal.1[literal.1.len() - 3].clone())
                        .with_message(format!("This has the type of {:?}", type_of(&par_arr[0])))
                        .with_color(Color::Red),
                )
                .with_source(Source::from(literal.0))
                .finish()]);
        }
    });

    data.push(result);
    Ok(data)
}

fn div(_state: &State, mut data: WTokens) -> Result<WTokens, Vec<Report>> {
    let par_arr = data.get_par(2);
    let full = [
        data.clone(),
        par_arr.iter().cloned().rev().collect::<Vec<_>>(),
        vec![Token::Function("div".to_string())],
    ]
    .concat();
    let parameters: (Token, Token) = par_arr.clone().iter().cloned().collect_tuple().unwrap();

    let result = Value(match parameters {
        (Value(x), Value(y)) => y / x,
        _ => {
            let literal = full.literal_enumerate();

            return Err(vec![Report::build(ReportKind::Error, (), 0)
                .with_message("Invalid Operation")
                .with_label(
                    Label::new(literal.1[literal.1.len() - 2].clone())
                        .with_message(format!("This has the type of {:?}", type_of(&par_arr[1])))
                        .with_color(Color::Red),
                )
                .with_label(
                    Label::new(literal.1[literal.1.len() - 3].clone())
                        .with_message(format!("This has the type of {:?}", type_of(&par_arr[0])))
                        .with_color(Color::Red),
                )
                .with_source(Source::from(literal.0))
                .finish()]);
        }
    });

    data.push(result);
    Ok(data)
}

fn modulo(_state: &State, mut data: WTokens) -> Result<WTokens, Vec<Report>> {
    let par_arr = data.get_par(2);
    let full = [
        data.clone(),
        par_arr.iter().cloned().rev().collect::<Vec<_>>(),
        vec![Token::Function("modulo".to_string())],
    ]
    .concat();
    let parameters: (Token, Token) = par_arr.clone().iter().cloned().collect_tuple().unwrap();

    let result = Value(match parameters {
        (Value(x), Value(y)) => y % x,
        _ => {
            let literal = full.literal_enumerate();

            return Err(vec![Report::build(ReportKind::Error, (), 0)
                .with_message("Invalid Operation")
                .with_label(
                    Label::new(literal.1[literal.1.len() - 2].clone())
                        .with_message(format!("This has the type of {:?}", type_of(&par_arr[1])))
                        .with_color(Color::Red),
                )
                .with_label(
                    Label::new(literal.1[literal.1.len() - 3].clone())
                        .with_message(format!("This has the type of {:?}", type_of(&par_arr[0])))
                        .with_color(Color::Red),
                )
                .with_source(Source::from(literal.0))
                .finish()]);
        }
    });

    data.push(result);
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
    let full = [
        data.clone(),
        vec![x.clone(), Token::Function("elem".to_string())],
    ]
    .concat();
    let y = match x {
        Group(content) => content,
        _ => {
            let literal = full.literal_enumerate();
            return Err(vec![Report::build(ReportKind::Error, (), 0)
                .with_message("Incorrect Type")
                .with_label(
                    Label::new(literal.1[literal.1.len() - 2].clone())
                        .with_message(format!(
                            "This has the type of {:?} but expected a Group",
                            type_of(&x)
                        ))
                        .with_color(Color::Red),
                )
                .with_source(Source::from(literal.0))
                .finish()]);
        }
    };

    Ok(y.iter().rev().cloned().collect::<Vec<_>>())
}

fn elem(_state: &State, mut data: WTokens) -> Result<WTokens, Vec<Report>> {
    let x = &data.get_par(1)[0];
    let full = [
        data.clone(),
        vec![x.clone(), Token::Function("elem".to_string())],
    ]
    .concat();
    data.reverse();

    let mut extracted = match x {
        Token::Range(Range::Full(range)) => data.splice(range.clone(), vec![]).collect::<WTokens>(),
        _ => {
            let literal = full.literal_enumerate();
            return Err(vec![Report::build(ReportKind::Error, (), 0)
                .with_message("Incorrect Type")
                .with_label(
                    Label::new(literal.1[literal.1.len() - 2].clone())
                        .with_message(format!(
                            "This has the type of {:?} but expected a Range",
                            type_of(&x)
                        ))
                        .with_color(Color::Red),
                )
                .with_source(Source::from(literal.0))
                .finish()]);
        }
    };

    extracted.reverse();
    data.reverse();
    data.append(&mut extracted);
    Ok(data)
}

fn copy_elem(_state: &State, mut data: WTokens) -> Result<WTokens, Vec<Report>> {
    let x = &data.get_par(1)[0];
    let full = [
        data.clone(),
        vec![x.clone(), Token::Function("copy_elem".to_string())],
    ]
    .concat();
    let mut data_clone: WTokens = data.clone();
    data_clone.reverse();

    let mut extracted = match x {
        Token::Range(Range::Full(range)) => data_clone
            .splice(range.clone(), vec![])
            .collect::<WTokens>(),
        _ => {
            let literal = full.literal_enumerate();
            return Err(vec![Report::build(ReportKind::Error, (), 0)
                .with_message("Incorrect Type")
                .with_label(
                    Label::new(literal.1[literal.1.len() - 2].clone())
                        .with_message(format!(
                            "This has the type of {:?} but expected a Range",
                            type_of(&x)
                        ))
                        .with_color(Color::Red),
                )
                .with_source(Source::from(literal.0))
                .finish()]);
        }
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
    let full = [
        data.clone(),
        par_arr.iter().cloned().rev().collect::<Vec<_>>(),
        vec![Token::Function("eq".to_string())],
    ]
    .concat();
    let parameters: (Token, Token) = par_arr.iter().cloned().collect_tuple().unwrap();

    let result = match parameters {
        (Value(x), Value(y)) => x == y,
        (Function(x), Function(y)) | (FunctionLiteral(x), FunctionLiteral(y)) => x == y,
        (Container(x), Container(y))
        | (Atom(x), Atom(y))
        | (ContainerLiteral(x), ContainerLiteral(y)) => x == y,
        (Parameter(x), Parameter(y)) => x == y,
        _ => {
            let literal = full.literal_enumerate();

            return Err(vec![Report::build(ReportKind::Error, (), 0)
                .with_message("Invalid Operation")
                .with_label(
                    Label::new(literal.1[literal.1.len() - 2].clone())
                        .with_message(format!("This has the type of {:?}", type_of(&par_arr[1])))
                        .with_color(Color::Red),
                )
                .with_label(
                    Label::new(literal.1[literal.1.len() - 3].clone())
                        .with_message(format!("This has the type of {:?}", type_of(&par_arr[0])))
                        .with_color(Color::Red),
                )
                .with_source(Source::from(literal.0))
                .finish()]);
        }
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
    let full = [
        data.clone(),
        par_arr.iter().cloned().rev().collect::<Vec<_>>(),
        vec![Token::Function("or".to_string())],
    ]
    .concat();
    let parameters: (Token, Token) = par_arr.iter().cloned().collect_tuple().unwrap();

    let token = Value(match parameters {
        (Value(x), Value(y)) => match (x != 0.0) || (y != 0.0) {
            true => 1.0,
            _ => 0.0,
        },
        _ => {
            let literal = full.literal_enumerate();

            return Err(vec![Report::build(ReportKind::Error, (), 0)
                .with_message("Invalid Operation")
                .with_label(
                    Label::new(literal.1[literal.1.len() - 2].clone())
                        .with_message(format!("This has the type of {:?}", type_of(&par_arr[1])))
                        .with_color(Color::Red),
                )
                .with_label(
                    Label::new(literal.1[literal.1.len() - 3].clone())
                        .with_message(format!("This has the type of {:?}", type_of(&par_arr[0])))
                        .with_color(Color::Red),
                )
                .with_source(Source::from(literal.0))
                .finish()]);
        }
    });

    data.push(token);
    Ok(data)
}

fn not(_state: &State, mut data: WTokens) -> Result<WTokens, Vec<Report>> {
    let par = &data.get_par(1)[0];
    let full = [
        data.clone(),
        vec![par.clone(), Token::Function("not".to_string())],
    ]
    .concat();

    let token = Value(match par {
        Value(x) => {
            if *x == 0.0 {
                1.0
            } else {
                0.0
            }
        }
        _ => {
            let literal = full.literal_enumerate();
            return Err(vec![Report::build(ReportKind::Error, (), 0)
                .with_message("Incorrect Type")
                .with_label(
                    Label::new(literal.1[literal.1.len() - 2].clone())
                        .with_message(format!(
                            "This has the type of {:?} but expected a Value",
                            type_of(&par)
                        ))
                        .with_color(Color::Red),
                )
                .with_source(Source::from(literal.0))
                .finish()]);
        }
    });

    data.push(token);
    Ok(data)
}

fn and(_state: &State, mut data: WTokens) -> Result<WTokens, Vec<Report>> {
    let par_arr = data.get_par(2);
    let full = [
        data.clone(),
        par_arr.iter().cloned().rev().collect::<Vec<_>>(),
        vec![Token::Function("and".to_string())],
    ]
    .concat();
    let parameters: (Token, Token) = par_arr.iter().cloned().collect_tuple().unwrap();

    let token = Value(match parameters {
        (Value(x), Value(y)) => match (x != 0.0) && (y != 0.0) {
            true => 1.0,
            _ => 0.0,
        },
        _ => {
            let literal = full.literal_enumerate();

            return Err(vec![Report::build(ReportKind::Error, (), 0)
                .with_message("Invalid Operation")
                .with_label(
                    Label::new(literal.1[literal.1.len() - 2].clone())
                        .with_message(format!("This has the type of {:?}", type_of(&par_arr[1])))
                        .with_color(Color::Red),
                )
                .with_label(
                    Label::new(literal.1[literal.1.len() - 3].clone())
                        .with_message(format!("This has the type of {:?}", type_of(&par_arr[0])))
                        .with_color(Color::Red),
                )
                .with_source(Source::from(literal.0))
                .finish()]);
        }
    });

    data.push(token);
    Ok(data)
}

fn greater(_state: &State, mut data: WTokens) -> Result<WTokens, Vec<Report>> {
    let par_arr = data.get_par(2);
    let full = [
        data.clone(),
        par_arr.iter().cloned().rev().collect::<Vec<_>>(),
        vec![Token::Function("greater".to_string())],
    ]
    .concat();
    let parameters: (Token, Token) = par_arr.iter().cloned().collect_tuple().unwrap();

    let token = Value(match parameters {
        (Value(x), Value(y)) => match y > x {
            true => 1.0,
            _ => 0.0,
        },
        _ => {
            let literal = full.literal_enumerate();

            return Err(vec![Report::build(ReportKind::Error, (), 0)
                .with_message("Invalid Operation")
                .with_label(
                    Label::new(literal.1[literal.1.len() - 2].clone())
                        .with_message(format!("This has the type of {:?}", type_of(&par_arr[1])))
                        .with_color(Color::Red),
                )
                .with_label(
                    Label::new(literal.1[literal.1.len() - 3].clone())
                        .with_message(format!("This has the type of {:?}", type_of(&par_arr[0])))
                        .with_color(Color::Red),
                )
                .with_source(Source::from(literal.0))
                .finish()]);
        }
    });

    data.push(token);
    Ok(data)
}

fn less(_state: &State, mut data: WTokens) -> Result<WTokens, Vec<Report>> {
    let par_arr = data.get_par(2);
    let full = [
        data.clone(),
        par_arr.iter().cloned().rev().collect::<Vec<_>>(),
        vec![Token::Function("less".to_string())],
    ]
    .concat();
    let parameters: (Token, Token) = par_arr.iter().cloned().collect_tuple().unwrap();

    let token = Value(match parameters {
        (Value(x), Value(y)) => match y < x {
            true => 1.0,
            _ => 0.0,
        },
        _ => {
            let literal = full.literal_enumerate();

            return Err(vec![Report::build(ReportKind::Error, (), 0)
                .with_message("Invalid Operation")
                .with_label(
                    Label::new(literal.1[literal.1.len() - 2].clone())
                        .with_message(format!("This has the type of {:?}", type_of(&par_arr[1])))
                        .with_color(Color::Red),
                )
                .with_label(
                    Label::new(literal.1[literal.1.len() - 3].clone())
                        .with_message(format!("This has the type of {:?}", type_of(&par_arr[0])))
                        .with_color(Color::Red),
                )
                .with_source(Source::from(literal.0))
                .finish()]);
        }
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
    let full = [
        data.clone(),
        par_arr.iter().cloned().rev().collect::<Vec<_>>(),
        vec![Token::Function("expand".to_string())],
    ]
    .concat();
    let parameters: (Token, Token) = par_arr.iter().cloned().collect_tuple().unwrap();

    match parameters {
        (Value(n), Group(group)) => {
            for _ in 0..n as usize {
                data.append(&mut group.clone())
            }
        }
        _ => {
            let literal = full.literal_enumerate();

            return Err(vec![Report::build(ReportKind::Error, (), 0)
                .with_message("Invalid Operation")
                .with_label(
                    Label::new(literal.1[literal.1.len() - 2].clone())
                        .with_message(format!(
                            "This has the type of {:?} expected Group",
                            type_of(&par_arr[1])
                        ))
                        .with_color(Color::Red),
                )
                .with_label(
                    Label::new(literal.1[literal.1.len() - 3].clone())
                        .with_message(format!(
                            "This has the type of {:?} expected Value",
                            type_of(&par_arr[0])
                        ))
                        .with_color(Color::Red),
                )
                .with_source(Source::from(literal.0))
                .finish()]);
        }
    }

    Ok(data)
}

fn release(_state: &State, mut data: WTokens) -> Result<WTokens, Vec<Report>> {
    let x = &data.get_par(1)[0];
    let full = [
        data.clone(),
        vec![x.clone(), Token::Function("release".to_string())],
    ]
    .concat();

    match x {
        Group(group) => data.append(&mut group.clone()),
        _ => {
            let literal = full.literal_enumerate();

            return Err(vec![Report::build(ReportKind::Error, (), 0)
                .with_message("Invalid Type")
                .with_label(
                    Label::new(literal.1[literal.1.len() - 1].clone())
                        .with_message(format!(
                            "This has the type of {:?} expected Group",
                            type_of(&x)
                        ))
                        .with_color(Color::Red),
                )
                .with_source(Source::from(literal.0))
                .finish()]);
        }
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
    "type" => type_of_container,
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
