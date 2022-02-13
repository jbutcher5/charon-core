#[derive(Debug)]
enum Token {
    Value(f64),
    Function(String),
}

fn lexer(code: &str) -> Vec<Vec<Token>> {
    vec![code
        .split(" ")
        .map(|x| match x.parse::<f64>() {
            Ok(n) => Token::Value(n),
            Err(_) => Token::Function(x.to_string()),
        })
        .collect()]
}

fn main() {
    println!("{:#?}", lexer("1 2 +"));
}
