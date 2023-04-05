use std::ops::Add;

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
}

fn replace_suffix(str: String, txt: &str, replacement: &str) -> String {
    if str.ends_with(txt) {
        let slice = &str[0..(str.len() - txt.len())];
        let result = String::from(slice).add(replacement);
        return result
    }
    str
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
}
