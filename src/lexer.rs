use crate::models::Token;
use lazy_static::lazy_static;
use logos::{Lexer, Logos};
use phf::phf_map;
use regex::{Captures, Regex};

static MACROS: phf::Map<&'static str, &'static str> = phf_map! {
    "TRUE" => "1",
    "FALSE" => "0"
};

pub fn macros(text: String) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\S*)").unwrap();
    }

    RE.replace_all(&text, |caps: &Captures| match MACROS.get(&caps[0]) {
        Some(value) => value.to_string(),
        _ => caps[0].to_string(),
    })
    .to_string()
}

pub fn expand_string(text: String) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"(.*)"(.*)"(.*)"#).unwrap();
    }

    RE.replace_all(&text, |caps: &Captures| {
        let collection = caps[2]
            .chars()
            .fold("".to_string(), |acc, x| format!("{} '{}'", acc, x));
        format!("{}{{ {} }}{}", &caps[1], &collection[1..], &caps[3])
    })
    .to_string()
}

pub fn expand_bracket(text: String) -> String {
    lazy_static! {
        static ref RE: Vec<Regex> = [
            r"(\S+)\((.*)|(.*)\((\S+)",
            r"(\S+)\)(.*)|(.*)\)(\S+)",
            r"(\S+)\{(.*)|(.*)\{(\S+)",
            r"(\S+)\}(.*)|(.*)\}(\S+)"
        ]
        .iter()
        .map(|x| Regex::new(x).unwrap())
        .collect();
    }

    let results = ["(", ")", "{", "}"]
        .iter()
        .zip(RE.iter())
        .map(|symbol| (symbol.0, symbol.1.captures(&text)))
        .filter(|result| result.1.is_some())
        .map(|result| (result.0, result.1.unwrap()))
        .collect::<Vec<_>>();

    if let Some(captures) = results.get(0) {
        let groups: Vec<String> = [1, 2]
            .iter()
            .map(|&x| {
                let range = match captures.1.get(x) {
                    Some(y) => y,
                    None => captures.1.get(x + 2).unwrap(),
                };
                text[range.start()..range.end()].to_string()
            })
            .collect();

        let mut whitespace = ("", "");

        if let Some(suffix) = groups[0].chars().last() {
            if !suffix.is_whitespace() {
                whitespace.0 = " ";
            }
        }

        if let Some(prefix) = groups[1].chars().next() {
            if !prefix.is_whitespace() {
                whitespace.1 = " ";
            }
        }

        let result = format!(
            "{}{}{}{}{}",
            groups[0], whitespace.0, captures.0, whitespace.1, groups[1]
        );
        let match_range = captures.1.get(0).unwrap();
        let mut new_text = text.clone();
        new_text.replace_range(match_range.start()..match_range.end(), &result);

        expand_bracket(new_text)
    } else {
        text
    }
}

fn string(lex: &mut Lexer<LexerToken>) -> Token {
    let slice = lex.slice();

    Token::Group(
        slice[1..slice.len() - 1]
            .chars()
            .map(Token::Char)
            .collect::<Vec<_>>(),
    )
}

fn container_literal(lex: &mut Lexer<LexerToken>) -> Token {
    let slice = lex.slice();

    Token::ContainerLiteral(
        slice[1..slice.len() - 1].to_string()
    )
}

fn boolean_guard(lex: &mut Lexer<LexerToken>) -> String {
    let slice = lex.slice().trim();

    slice[..slice.len() - 4].to_string()
}

fn guard_option(lex: &mut Lexer<LexerToken>) -> String {
    let slice = lex.slice();

    slice[..slice.len() - 4].to_string()
}

fn assignment(lex: &mut Lexer<LexerToken>) -> String {
    let slice = lex.slice().trim();

    slice[..slice.len() - 3].to_string()
}

#[derive(Logos, Debug, Clone, PartialEq)]
pub enum LexerToken {
    #[regex(r"\S+ <-\| *", boolean_guard)]
    BooleanGuard(String),

    #[regex(r"\S+ -> ", guard_option)]
    GuardOption(String),

    #[regex(r"\S+ <- *", assignment)]
    Assignment(String),

    #[regex("\"[^\"]*\"", string)]
    #[regex(r"-?\d+(\.\d+)?", |number| Token::Value(number.slice().parse().unwrap()))]
    #[regex("'.'", |character| Token::Char(character.slice().chars().nth(1).unwrap()))]
    #[regex("`[^`]*`", container_literal)]
    #[token("TRUE", |_| Token::Value(1.0))]
    #[token("FALSE", |_| Token::Value(0.0))]
    Token(Token),

    #[token("\n  ")]
    Indent,

    #[token(" ")]
    Seperator,

    #[token("\n")]
    Newline,

    #[error]
    Error,
}
