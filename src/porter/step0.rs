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
    fn test_trim_s_apostrophe() {
        let array = vec![("te'st", "te'st"), ("test's", "test")];
        for (word, expected) in array {
            let subject = trim_s_apostrophe(word.to_string());
            assert_eq!(expected.to_string(), subject);
        }
    }
}
