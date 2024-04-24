// Improvised CSS parser.

#[derive(Debug, PartialEq)]
pub struct Rule {
    pub selector: String,
    pub properties: Vec<(String, String)>,
}

#[derive(Debug)]
pub struct ParseError {
    message: String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub fn parse(input: &str) -> Result<Vec<Rule>, ParseError> {
    let mut output = Vec::new();

    let mut block = false;
    let mut property = false;
    let mut needle = String::from("");
    let mut properties = Vec::new();

    for char in input.chars() {
        if !block {
            if char.is_alphabetic() {
                needle.push(char);
            }

            if char.is_whitespace() {
                if !needle.is_empty() {
                    output.push(Rule {
                        selector: needle.clone(),
                        properties,
                    });
                    needle.clear();
                    properties = Vec::new();
                }
            }
        } else {
            // we're parsing property blocks
            if char.is_alphabetic() || char == '-' || char == '_' || char.is_numeric() {
                needle.push(char);
            }
            if char == ':' {
                property = true;
            }
            if char.is_whitespace() && property {
                property = false;
                properties.push((needle.clone(), String::from("")));
                needle.clear();
            }
        }
        match char {
            '{' => block = true,
            '}' => {
                block = false;
                if !needle.is_empty() {
                    properties.last_mut().unwrap().1 = needle.clone();
                    needle.clear();
                }
            }
            _ => {}
        }
    }

    Ok(output)
}

