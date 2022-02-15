use phf::phf_map;
use crate::{WCode, WFunc, as_nums, as_wcode};

fn sum(data: WCode) -> WCode {
    let nums = as_nums(data);
    as_wcode(vec![nums.iter().sum()])
}

pub static FUNCTIONS: phf::Map<&'static str, WFunc> = phf_map! {
    "sum" => sum
};
