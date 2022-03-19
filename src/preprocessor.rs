use regex::Regex;
use lazy_static::lazy_static;

pub fn expand_bracket(text: String) -> String {
    lazy_static! {
        static ref OPEN: Regex = Regex::new(r"(\S+)\((.*)|(.*)\((\S+)").unwrap();
        static ref CLOSE: Regex = Regex::new(r"(\S+)\)(.*)|(.*)\)(\S+)").unwrap();
    }

    if let Some(captures) = OPEN.captures(&text) {
        let groups: Vec<String> = [1, 2].iter().map(|&x| {
            let range = match captures.get(x) {
                Some(y) => y,
                None => captures.get(x+2).unwrap()
            };
            text[range.start()..range.end()].to_string()
        }).collect();

        let result = format!("{} ( {}", groups[0], groups[1]);
        let match_range = captures.get(0).unwrap();
        let mut new_text = text.clone();
        new_text.replace_range(match_range.start()..match_range.end(), &result);

        expand_bracket(new_text)
    } else if let Some(captures) = CLOSE.captures(&text) {
        let groups: Vec<String> = [1, 2].iter().map(|&x| {
            let range = match captures.get(x) {
                Some(y) => y,
                None => captures.get(x+2).unwrap()
            };
            text[range.start()..range.end()].to_string()
        }).collect();

        let result = format!("{} ) {}", groups[0], groups[1]);
        let match_range = captures.get(0).unwrap();
        let mut new_text = text.clone();
        new_text.replace_range(match_range.start()..match_range.end(), &result);

        expand_bracket(new_text)
    } else {
        text
    }
}
