use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Variables(HashMap<String, String>);

impl Variables {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn set<K: Into<String>, V: ToString>(&mut self, key: K, value: V) -> &mut Self {
        self.0.insert(key.into(), value.to_string());
        self
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(|s| s.as_str())
    }

    pub fn substitute(&self, s: &'static str) -> String {
        // Wanted to use nom, but maaan.. :P Co-author: github co-pilot
        let mut result = String::new();
        let mut chars = s.chars().peekable();
        'outer: while let Some(c) = chars.next() {
            if c == '{' {
                match chars.peek() {
                    Some('}') | Some('{') => {
                        result.push('{');
                        result.push(chars.next().unwrap());
                        continue;
                    }
                    None => {
                        result.push('{');
                        return result;
                    }
                    _ => {}
                }

                let mut var = String::new();
                while let Some(c) = chars.next() {
                    if c == '}' {
                        break;
                    }
                    if !c.is_alphanumeric() && c != '_' && c != '[' && c != ']' && c != '.' {
                        result.push('{');
                        result.push_str(&var);
                        result.push(c);
                        // Start over
                        continue 'outer;
                    }
                    var.push(c);
                }
                let Some(var) = self.0.get(&var) else {
                    result.push('{');
                    result.push_str(&var);
                    result.push('}');
                    continue;
                };

                result.push_str(var);
            } else {
                result.push(c);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut vars = Variables::new();
        vars.set("name", "World")
            .set("bang", "!")
            .set("complex_var.is[x]", "COMPLEX");

        let cases = [
            ("Hello {name}!", "Hello World!"),
            ("Hello {name}-{name}{bang}!", "Hello World-World!!"),
            ("Hello {name}-{{name}{bang}", "Hello World-{{name}!"),
            ("Hello {name}-{", "Hello World-{"),
            ("Hello {bad-char}", "Hello {bad-char}"),
            ("Hello {not_exist}", "Hello {not_exist}"),
            ("{}", "{}"),
            ("{", "{"),
            ("{name }{name}{name }", "{name }World{name }"),
            ("{complex_var.is[x]} {name}", "COMPLEX World"),
        ];

        for (input, expected) in cases.iter() {
            let result = vars.substitute(input);
            assert_eq!(result, *expected, "{} failed", input);
        }
    }
}
