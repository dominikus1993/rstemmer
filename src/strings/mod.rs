use std::ops::Add;

pub fn replace_suffix(str: String, txt: &str, replacement: &str) -> String {
    if str.ends_with(txt) {
        let slice = &str[0..(str.len() - txt.len())];
        let result = String::from(slice).add(replacement);
        return result
    }
    str
} 

pub fn remove_suffix(str: String, txt: &str) -> String {
    if str.ends_with(txt) {
        let slice = &str[0..(str.len() - txt.len())];
        let result = String::from(slice);
        return result
    }
    str
} 