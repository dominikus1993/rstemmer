use regex::Regex;

fn trim_end_apostrophe(str: String) -> String {
    match str.strip_suffix('\'') {
        Some(word) => word.to_string(),
        None => str
    }
}

fn trim_start_apostrophe(str: String) -> String {
    match str.strip_prefix('\'') {
        Some(word) => word.to_string(),
        None => str
    }
}

fn trim_s_apostrophe(str: String) -> String {
    match str.strip_suffix("'s") {
        Some(word) => word.to_string(),
        None => str
    }
}

fn mark_consonant_y(word: String) -> String {
    let re = Regex::new(r"^y|([aeiou])y").unwrap();

    if word.contains('y') {
        re.replace_all(&word, |caps: &regex::Captures| {
            if let Some(mat) = caps.get(1) {
                format!("{}Y", mat.as_str())
            } else {
                "Y".to_string()
            }
        }).to_string()
    } else {
        word
    }
}

pub fn apply(word: String) -> String {
    mark_consonant_y(trim_s_apostrophe(trim_end_apostrophe(trim_start_apostrophe(word))))
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_trim_end_apostrophe() {
        let array = vec![("te'st", "te'st"), ("test'", "test")];
        for (word, expected) in array {
            let subject = trim_end_apostrophe(word.to_string());
            assert_eq!(expected.to_string(), subject);
        }
    }

    #[test]
    fn test_trim_start_apostrophe() {
        let array = vec![("te'st", "te'st"), ("'test", "test")];
        for (word, expected) in array {
            let subject = trim_start_apostrophe(word.to_string());
            assert_eq!(expected.to_string(), subject);
        }
    }

    #[test]
    fn test_mark_consonant_y() {
        let array = vec![("te'st", "te'st"), ("test's", "test")];
        for (word, expected) in array {
            let subject = trim_s_apostrophe(word.to_string());
            assert_eq!(expected.to_string(), subject);
        }
    }

    #[test]
    fn test_trim_s_apostrophe() {
        let array = vec![("flying", "flying"), ("youth", "Youth"), ("boyish", "boYish")];
        for (word, expected) in array {
            let subject = mark_consonant_y(word.to_string());
            assert_eq!(expected.to_string(), subject);
        }
    }

    #[test]
    fn test_apply() {
        let array = vec![("youth's'", "Youth")];
        for (word, expected) in array {
            let subject = apply(word.to_string());
            assert_eq!(expected.to_string(), subject);
        }
    }
}
