#[derive(Debug, Copy, Clone)]
enum Token {
    Value(f64),
    Function(fn(WCode) -> WCode),
}

type WCode = Vec<Vec<Token>>;

fn as_nums(data: WCode) -> Vec<Vec<f64>> {
    data.iter()
        .map(|arr| {
            arr.iter()
                .map(|&value| match value {
                    Token::Value(n) => n,
                    Token::Function(_) => 1.0,
                })
                .collect()
        })
        .collect()
}

fn as_wcode(data: Vec<Vec<f64>>) -> WCode {
    data.iter()
        .map(|arr| arr.iter().map(|&value| Token::Value(value)).collect())
        .collect()
}

fn has_function(data: Vec<Vec<Token>>) -> bool {
    for arr in data {
        for token in arr {
            match token {
                Token::Function(_) => return true,
                _ => continue
            }
        }
    }

    false
}

fn sum(data: WCode) -> WCode {
    let nums = as_nums(data);
    as_wcode(vec![nums.iter().map(|arr| arr.iter().sum()).collect()])
}

fn evaluate(data: WCode) -> WCode {
    let mut new_code = data.clone();

    let final_token: Option<Token> = match new_code.first() {
        Some(_) => new_code[0].pop(),
        None => panic!("No code provided"),
    };

    let final_function: fn(WCode) -> WCode = match final_token {
        Some(token) => match token {
            Token::Function(func) => func,
            Token::Value(_) => return data,
        },
        None => panic!("No code provided"),
    };

    if new_code[0].iter().any(|x| match x {
        Token::Function(_) => true,
        _ => false
    }) {
        return final_function(evaluate(new_code));
    }

    final_function(new_code)
}

fn lexer(code: &str) -> WCode {
    vec![code
        .split(" ")
        .map(|x| match x.parse::<f64>() {
            Ok(n) => Token::Value(n),
            Err(_) => Token::Function(sum),
        })
        .collect()]
}

fn main() {
    println!("{:#?}", evaluate(lexer("1 2 3 3 4 ")));
}
