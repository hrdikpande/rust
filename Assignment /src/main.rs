use serde_json::{json, Value}; // to handel json
use std::collections::HashMap; // will store key value pairs


//define a enum
// defining erros which can occur while parrsing
#[derive(Debug)]
enum ParseError {
    InvalidFormat,
    InvalidType,
    InvalidBoolean,
    InvalidNumber,
}

//encoded str input
// result(hashmap) output
// Result either value or parseError
fn parse_encoded_string(encoded_str: &str) -> Result<Value, ParseError> {
    let mut result = HashMap::new();

    // Split using # string into key-value pairs
    let pairs: Vec<&str> = encoded_str.split('#').filter(|s| !s.is_empty()).collect();
     // non-empty parts ko pairs vector mein store karti hai.

    //for loop
    // pair ko | split kiya
    for pair in pairs {
        // parse the key-value pair
        let mut parts = pair.split('|');
        let key_type = parts.next().ok_or(ParseError::InvalidFormat)?;
        let value = parts.next().ok_or(ParseError::InvalidFormat)?;

        if key_type.len() < 3 {
            return Err(ParseError::InvalidFormat);
        }

        let is_array = key_type.chars().nth(0).unwrap();
        let value_type = key_type.chars().nth(1).unwrap();
        let key = &key_type[2..];

        match (is_array, value_type) {
            ('0', '0') => {
                // date
                result.insert(key.to_string(), json!(value));
            }
            ('0', '1') => {
                // number
                let number: f64 = value.parse().map_err(|_| ParseError::InvalidNumber)?;
                result.insert(key.to_string(), json!(number));
            }
            ('0', '2') => {
                // string
                result.insert(key.to_string(), json!(value));
            }
            ('0', '3') => {
                // bool
                let boolean = match value.to_lowercase().as_str() {
                    "y" | "t" => true,
                    "n" | "f" => false,
                    _ => return Err(ParseError::InvalidBoolean),
                };
                result.insert(key.to_string(), json!(boolean));
            }
            ('1', '0') => {
                // array of dates
                let dates: Vec<&str> = value.split(',').collect();
                result.insert(key.to_string(), json!(dates));
            }
            ('1', '1') => {
                // array of numbers
                let numbers: Result<Vec<f64>, _> = value.split(',').map(|v| v.parse()).collect();
                result.insert(key.to_string(), json!(numbers.map_err(|_| ParseError::InvalidNumber)?));
            }
            ('1', '2') => {
                // array of strings
                let strings: Vec<&str> = value.split(',').collect();
                result.insert(key.to_string(), json!(strings));
            }
            ('1', '3') => {
                // array of bools
                let booleans: Result<Vec<bool>, _> = value.split(',').map(|v| {
                    match v.to_lowercase().as_str() {
                        "y" | "t" => Ok(true),
                        "n" | "f" => Ok(false),
                        _ => Err(ParseError::InvalidBoolean),
                    }
                }).collect();
                result.insert(key.to_string(), json!(booleans?));
            }
            _ => return Err(ParseError::InvalidType),
        }
    }

    Ok(json!(result)) // return result in json format
}

fn main() {
    //encoded string
    let encoded_str = "#00date|1997-02-06#02name|bob#01age|20#03hasPassport|Y#12access|read_db,write_db,view_logs";

    match parse_encoded_string(encoded_str) {
        Ok(json) => {
            println!("Parsed JSON: {}", json.to_string());
        }
        Err(e) => {
            eprintln!("Error parsing encoded string: {:?}", e);
        }
    }
}
