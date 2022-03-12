use crate::models::{State, WCode, WFuncVariant, WTokens, Token};
use crate::utils::{first_special_instance, outter_function, special_pairs, WFunc};

pub trait WEval {
    fn wsection_eval(&mut self, data: Vec<WCode>) -> Vec<WTokens>;
    fn eval(&self, data: WTokens) -> WTokens;
}

impl WEval for State {
    fn wsection_eval(&mut self, data: Vec<WCode>) -> Vec<WTokens> {
        let mut result: Vec<WTokens> = Vec::new();

        for section in data {
            match section.container {
                Some(container) => {
                    let mut cases = vec![(vec![Token::Value(1.0)], section.default_case)];

                    if let Some(container_cases) = section.cases {
                        cases.append(&mut container_cases)
                    }

                    self.insert(container, cases);
                }
                None => result.push(self.eval(section.default_case)),
            }
        }

        result
    }

    fn eval(&self, data: WTokens) -> WTokens {
        let mut new_code = data.clone();

        let first = first_special_instance("(".to_string(), &new_code);
        let second = match first {
            Some(x) => special_pairs(("(".to_string(), ")".to_string()), &new_code, &x),
            None => None,
        };

        if first.is_some() && second.is_some() {
            let (x, y) = (first.unwrap(), second.unwrap());
            let bracket_code = &data[x + 1..y];
            new_code.splice(x..y + 1, self.eval(bracket_code.to_vec()));

            if first_special_instance("(".to_string(), &new_code).is_some() {
                new_code = self.eval(new_code)
            }
        }

        let funcs = outter_function(&new_code);

        match funcs {
            (Some((second_func_pos, _)), Some((first_func_pos, func))) => {
                let code_to_evaluate: WTokens = new_code[..first_func_pos].to_vec();

                let result = self.eval(match func {
                    WFuncVariant::Function(func) => func(code_to_evaluate),
                    WFuncVariant::Container(x) => {
                        self.apply(self.get(&x).unwrap(), &code_to_evaluate)
                    }
                });

                new_code.splice(..first_func_pos + 1, result);

                if first_func_pos != second_func_pos {
                    new_code = self.eval(new_code);
                }

                new_code
            }
            _ => new_code,
        }
    }
}
