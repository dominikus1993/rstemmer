fn trim_end_apostrophe(str: String) -> Option<String> {
    let res = str.strip_suffix('\'')?;
    Some(res.to_string())
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
            assert!(subject.is_some());
            assert_eq!(expected.to_string(), subject.unwrap());
        }
    }
}
