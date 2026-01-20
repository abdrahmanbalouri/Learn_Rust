use std::collections::HashMap;

pub fn counting_words(words: &str) -> HashMap<String, u32> {
    let mut k = HashMap::new();

    for i in words.split_whitespace() {
        let new = new(i);
        if new == "" {
            continue;
        }
        let count = k.entry(new).or_insert(0);
        *count += 1;
    }
    k
}

fn new(s: &str) -> String {
    let f = s.to_lowercase();
    let mut d = String::new();

    for (l, i) in f.chars().enumerate() {
        if i == '\'' && l != 0 && l != f.len() - 1 {
            d.push(i);
        } else if i.is_alphanumeric() {
            d.push(i);
        }
    }
    return d;
}
