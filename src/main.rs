#[derive(Debug, Clone)]
enum Token {
    Value(f64),
    Function(fn(WCode) -> WCode),
    FunctionLiteral(fn(WCode) -> WCode),
    Other(String),
}

type WCode = Vec<Token>;

fn as_nums(arr: WCode) -> Vec<f64> {
    arr.iter()
        .map(|value| match value.clone() {
            Token::Value(n) => n,
            _ => 1.0,
        })
        .collect()
}

fn as_wcode(arr: Vec<f64>) -> WCode {
    arr.iter().map(|&value| Token::Value(value)).collect()
}

fn last_function(arr: &WCode) -> Option<(usize, fn(WCode) -> WCode)> {
    let reversed = arr.iter().rev();

    for (i, token) in reversed.enumerate() {
        match token {
            Token::Function(value) => return Some((arr.len() - (i + 1), *value)),
            _ => continue,
        }
    }

    None
}

fn get_first_bracket_open(arr: &WCode) -> Option<usize> {
    for (i, token) in arr.iter().enumerate() {
        match token {
            Token::Other(value) => {
                if value == "(" {
                    return Some(i);
                } else {
                    continue;
                }
            }
            _ => continue,
        }
    }

    None
}

fn get_last_bracket_close(arr: &WCode) -> Option<usize> {
    let reversed = arr.iter().rev();

    for (i, token) in reversed.enumerate() {
        match token {
            Token::Other(value) => {
                if value == ")" {
                    return Some(arr.len() - (i + 1));
                } else {
                    continue;
                }
            }
            _ => continue,
        }
    }

    None
}

fn sum(data: WCode) -> WCode {
    let nums = as_nums(data);
    vec![Token::Value(nums.iter().sum())]
}

fn evaluate(data: WCode) -> WCode {
    let mut new_code = data.clone();

    let brackets = (
        get_first_bracket_open(&new_code),
        get_last_bracket_close(&new_code),
    );

    if brackets.0.is_some() && brackets.1.is_some() {
        let (x, y) = (brackets.0.unwrap(), brackets.1.unwrap());
        let bracket_code = &data[x + 1..y];
        new_code.splice(x..y + 1, evaluate(bracket_code.to_vec()));
    }

    match last_function(&new_code) {
        Some((func_pos, func)) => {
            let code_to_evaluate: WCode = new_code[..func_pos].to_vec();
            let result = func(code_to_evaluate);

            new_code.splice(..func_pos + 1, result);
            new_code
        }
        None => new_code,
    }
}

fn lexer(code: &str) -> WCode {
    code.split(" ")
        .map(|x| match x.parse::<f64>() {
            Ok(n) => Token::Value(n),
            Err(_) => {
                let mut chars = x.chars();

                if x.len() > 2 && chars.nth(0).unwrap() == '`' && chars.last().unwrap() == '`' {
                    Token::FunctionLiteral(sum)
                } else if ["(", ")"].iter().any(|&y| x == y) {
                    Token::Other(x.to_string())
                } else {
                    Token::Function(sum)
                }
            }
        })
        .collect()
}

fn main() {
    println!("{:#?}", evaluate(lexer("1 2 3 3 ( 4 6 + ) sum")));
}
