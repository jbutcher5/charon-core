use crate::{as_nums, as_wcode, Token, WCode, WFunc};
use phf::phf_map;

fn sum(data: WCode) -> WCode {
    let nums = as_nums(data);
    as_wcode(vec![nums.iter().sum()])
}

fn add(mut data: WCode) -> WCode {
    if data.len() < 2 {
        panic!("Not enough parameters to call add!");
    }

    let x = as_nums(vec![data.pop().unwrap(), data.pop().unwrap()]);
    data.push(Token::Value(x.iter().sum()));
    data
}

fn sub(mut data: WCode) -> WCode {
    if data.len() < 2 {
        panic!("Not enough parameters to call sub!");
    }

    let x = as_nums(vec![data.pop().unwrap(), data.pop().unwrap()]);
    let result = x[1] - x[0];
    data.push(Token::Value(result));
    data
}

fn mul(mut data: WCode) -> WCode {
    if data.len() < 2 {
        panic!("Not enough parameters to call mul!");
    }

    let x = as_nums(vec![data.pop().unwrap(), data.pop().unwrap()]);
    let result = x[1] * x[0];
    data.push(Token::Value(result));
    data
}

fn div(mut data: WCode) -> WCode {
    if data.len() < 2 {
        panic!("Not enough parameters to call div!");
    }

    let x = as_nums(vec![data.pop().unwrap(), data.pop().unwrap()]);
    let result = x[1] / x[0];
    data.push(Token::Value(result));
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
    "/" => div
};
