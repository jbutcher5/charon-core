use crate::models::{Range, Token};
use itertools::Itertools;
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

fn string(lex: &mut Lexer<LexerToken>) -> Token {
    let slice = lex.slice();

    Token::Group(
        slice[1..slice.len() - 1]
            .chars()
            .map(Token::Char)
            .collect::<Vec<_>>(),
    )
}

fn container_literal(lex: &mut Lexer<LexerToken>) -> String {
    let mut slice = lex.slice().to_string();
    slice.retain(|c| c != '`');

    slice
}

fn boolean_guard(lex: &mut Lexer<LexerToken>) -> String {
    let slice = lex.slice().trim();

    slice[..slice.len() - 4].to_string()
}

fn guard_option(lex: &mut Lexer<LexerToken>) -> (String, String) {
    let slice = lex.slice().split(" -> ").collect::<Vec<_>>();

    (slice[0][1..].trim().to_string(), slice[1].to_string())
}

fn assignment(lex: &mut Lexer<LexerToken>) -> String {
    let slice = lex.slice().trim();

    slice[..slice.len() - 3].to_string()
}

fn slice_full(lex: &mut Lexer<LexerToken>) -> Token {
    let slice = &lex.slice()[1..];

    if let Some((end, start)) = slice
        .split("..")
        .map(|x| x.parse::<usize>().unwrap())
        .collect_tuple()
    {
        Token::Parameter(Range::Full(start..=end))
    } else {
        panic!("Invalid tuple found.")
    }
}

fn slice_to(lex: &mut Lexer<LexerToken>) -> Token {
    let mut slice = lex.slice()[1..].to_string();
    slice.retain(|c| c != '.');

    Token::Parameter(Range::From(..slice.parse::<usize>().unwrap()))
}

fn slice_from(lex: &mut Lexer<LexerToken>) -> Token {
    let mut slice = lex.slice()[1..].to_string();
    slice.retain(|c| c != '.');

    Token::Parameter(Range::To(slice.parse::<usize>().unwrap()..))
}

fn slice_at(lex: &mut Lexer<LexerToken>) -> Token {
    let slice = lex.slice()[1..].parse::<usize>().unwrap();
    Token::Parameter(Range::Full(slice..=slice))
}

#[derive(Logos, Debug, Clone, PartialEq)]
pub enum LexerToken {
    #[regex(r"[a-zA-Z] <-\|", boolean_guard)]
    BooleanGuard(String),

    #[regex(r"\n  [^\n]*", |default| default.slice()[1..].trim().to_string())]
    GuardDefault(String),

    #[regex(r"\n  [^\n]+ -> [^\n]*", guard_option)]
    GuardOption((String, String)),

    #[regex(r"[a-zA-Z]+ <- *", assignment)]
    Assignment(String),

    #[regex("\"[^\"]*\"", string)]
    #[regex(r"-?\d+(\.\d+)?", |number| Token::Value(number.slice().parse().unwrap()))]
    #[regex("'.'", |character| Token::Char(character.slice().chars().nth(1).unwrap()))]
    #[regex(r"\$\d+\.\.\d+", slice_full)]
    #[regex(r"\$\d+\.\.", slice_to)]
    #[regex(r"\$\.\.\d+", slice_from)]
    #[regex(r"\$\d+", slice_at)]
    #[regex(r":[a-zA-Z\+\-\*/%><\|&]+", |atom| Token::Atom(atom.slice()[1..].to_string()))]
    #[regex(r"\(|\)|\{|\}", |s| Token::Special(s.slice().to_string()))]
    Token(Token),

    #[regex(r"[a-zA-Z\+\-\*/%><\|&]+", |func| func.slice().to_string())]
    Function(String),

    #[regex(r"`[a-zA-Z\+\-\*/%><\|&]+`", container_literal)]
    FunctionLiteral(String),

    #[token(" ")]
    Seperator,

    #[token("\n")]
    Newline,

    #[error]
    Error,
}
