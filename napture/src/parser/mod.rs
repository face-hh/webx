// Improvised CSS parser.

use std::collections::HashMap;

#[derive(Debug)]
pub struct ParseError {
    message: String,
}

impl std::error::Error for ParseError {}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub fn parse(input: &str) -> Result<HashMap<String, HashMap<String, String>>, ParseError> {
    let mut result = HashMap::new();
    let mut current_selector: Option<String> = None;
    let mut current_declarations: HashMap<String, String> = HashMap::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("/*") && line.ends_with("*/") {
            continue;
        }

        if line.ends_with('{') {
            current_selector = Some(line.trim_end_matches('{').trim().to_string());
            current_declarations.clear();
        } else if line.ends_with('}') {
            if let Some(selector) = current_selector.take() {
                result
                    .entry(selector)
                    .and_modify(|val: &mut HashMap<String, String>| {
                        for (key, value) in current_declarations.iter() {
                            val.insert(key.clone(), value.clone());
                        }
                    })
                    .or_insert(current_declarations.clone());
            } else {
                return Err(ParseError {
                    message: String::from("Closing brace without opening selector"),
                });
            }
        } else {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() == 2 {
                let property = parts[0].trim().to_string();
                let value = parts[1].trim_end_matches(';').trim().to_string();
                current_declarations.insert(property, value);
            } else {
                return Err(ParseError {
                    message: String::from("Closing brace without opening selector"),
                });
            }
        }
    }

    if current_selector.is_some() {
        return Err(ParseError {
            message: String::from("Closing brace without opening selector"),
        });
    }

    Ok(result)
}
