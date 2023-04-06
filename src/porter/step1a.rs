use std::ops::Add;

use regex::Regex;

use crate::strings::{ replace_suffix, remove_suffix };

#[derive(PartialEq, Eq, Debug, Clone)]
enum ReplaceResult {
    Found(String),
    Next(String),
}

impl ReplaceResult {
    fn bind(&self, f: fn(String) -> ReplaceResult) -> ReplaceResult {
        match self {
            Self::Found(_) => self.clone(),
            Self::Next(str) => f(str.clone())
        }
    }

    fn value(&self) -> String {
        match self {
            Self::Found(str) =>str.clone(), 
            Self::Next(str) => str.clone()
        }
    }
}

fn remove_s(word: String) -> ReplaceResult {
    let re = Regex::new(r"([aeiou]).+s$").unwrap();
    if re.is_match(&word) {
        return ReplaceResult::Found(remove_suffix(word, "s"));
    }
    ReplaceResult::Next(word)
}

fn replace_ied_and_ies(word: String) -> ReplaceResult {
    if !(word.ends_with("ied") || word.ends_with("ies")) {
        return ReplaceResult::Next(word);
    }

    let replacement = if word.len() > 4 {
        "i"
    } else {
        "ie"
    };
    let res = replace_suffix(replace_suffix(word, "ied", replacement), "ies", replacement);
    return ReplaceResult::Found(res);
}

fn replace_sses(str: String) -> ReplaceResult {
    if str.ends_with("sses") {
        let result = replace_suffix(str.clone(), "sses", "ss");
        return ReplaceResult::Found(result);
    }
    ReplaceResult::Next(str)
}

fn leave_us_and_ss(word: String) -> ReplaceResult {
    if word.ends_with("ss") || word.ends_with("us") {
        return ReplaceResult::Found(word);
    }
    return ReplaceResult::Next(word);
}

pub fn apply(word: String) -> String {
    let result = replace_sses(word).bind(replace_ied_and_ies).bind(leave_us_and_ss).bind(remove_s);
    result.value()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_replace_sses() {
        let array = vec![("actresses", ReplaceResult::Found("actress".to_string()))];
        for (word, expected) in array {
            let subject = replace_sses(word.to_string());
            assert_eq!(expected, subject);
        }
    }

    #[test]
    fn test_leave_us_and_ss() {
        let array = vec![("abyss", ReplaceResult::Found("abyss".to_string())), ("us", ReplaceResult::Found("us".to_string())), ("gap", ReplaceResult::Next("gap".to_string()))];
        for (word, expected) in array {
            let subject = leave_us_and_ss(word.to_string());
            assert_eq!(expected, subject);
        }
    }

    #[test]
    fn test_replace_ied_and_ies() {
        let array = vec![("tied", ReplaceResult::Found("tie".to_string())), ("cries", ReplaceResult::Found("cri".to_string())), ("ties", ReplaceResult::Found("tie".to_string())), ("test", ReplaceResult::Next("test".to_string()))];
        for (word, expected) in array {
            let subject = replace_ied_and_ies(word.to_string());
            assert_eq!(expected, subject);
        }
    }

    #[test]
    fn test_remove_s() {
        let array = vec![("gas", ReplaceResult::Next("gas".to_string())), ("kiwis", ReplaceResult::Found("kiwi".to_string())), ("gaps", ReplaceResult::Found("gap".to_string())), ("test", ReplaceResult::Next("test".to_string()))];
        for (word, expected) in array {
            let subject = remove_s(word.to_string());
            assert_eq!(expected, subject);
        }
    }

    #[test]
    fn test_apply() {
        let array = vec![("actresses", "actress".to_string()), ("test", "test".to_string())];
        for (word, expected) in array {
            let subject = apply(word.to_string());
            assert_eq!(expected, subject);
        }
    }
}
