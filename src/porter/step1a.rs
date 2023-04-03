use std::ops::Add;

enum ReplaceResult {
    Found(String),
    Next(String),
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
        ReplaceResult::Found(result);
    }
    ReplaceResult::Next(str)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_replace_suffix() {
        let array = vec![("test", "st", "xt", "text")];
        for (word, txt, replacement, expected) in array {
            let subject = replace_suffix(word.to_string(), txt, replacement);
            assert_eq!(expected.to_string(), subject);
        }
    }
}
