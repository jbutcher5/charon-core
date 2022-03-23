use lazy_static::lazy_static;
use regex::Regex;

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
