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

pub fn parse(input: &str) -> Result<HashMap<String, Vec<(String, String)>>, ParseError> {
    let mut output = HashMap::new();
    let lines: Vec<&str> = input.lines().collect();

    let mut selectors = Vec::new();
    let mut props = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        let trimmed_line = line.trim();

        if trimmed_line.is_empty() {
            continue;
        }

        if trimmed_line.ends_with("{") {
            selectors = trimmed_line
                .trim_end_matches("{")
                .trim()
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
        } else if trimmed_line.ends_with("}") {
            for selector in selectors.iter() {
                output.insert(selector.clone(), props.clone());
            }

            selectors.clear();
            props.clear();
        } else {
            let parts: Vec<&str> = trimmed_line.split(":").collect();
            if parts.len() == 2 {
                let property = parts[0].trim().to_string();
                let value = parts[1].trim_end_matches(";").trim().to_string();

                props.push((property, value));
            } else {
                return Err(ParseError {
                    message: format!("Invalid property at line #{}, line contents: {}", i, line),
                });
            }
        }
    }

    Ok(output)
}



