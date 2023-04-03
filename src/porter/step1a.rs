enum ReplaceResult {
    Found(String),
    Next(String),
}

fn replace_suffix(str: String, txt: &str, replacement: &str) -> String {
    ""
} 

fn replace_sses(str: String) -> ReplaceResult {
    if str.ends_with("sses") {
        let result = replace_suffix(str, "sses", "ss");
        ReplaceResult::Found(result);
    }
    ReplaceResult::Next(str)
}