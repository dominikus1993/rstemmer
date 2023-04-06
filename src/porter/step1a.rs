use std::ops::Add;

use crate::strings::replace_suffix;

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
// }
// let replaceIedAndIes(word: string) =
// if word.EndsWith("ied") || word.EndsWith("ies") then
//     let result = if word.Length > 4 then
//                     word |> replaceSuffix "ied" "i" |> replaceSuffix "ies" "i"
//                  else
//                     word |> replaceSuffix "ied" "ie" |> replaceSuffix "ies" "ie"
//     Found(result)
// else
//     Next(word)

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
}
