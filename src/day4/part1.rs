use std::collections::HashMap;


pub fn is_passphrase_valid<S: Into<String>>(passphrase: S) -> Option<String> {
    let passphrase_string = passphrase.into();
    match passphrase_string
        .split_whitespace()
        .fold(HashMap::new(), |mut acc, word|{
            *acc.entry(word).or_insert(0) += 1;
            acc
        })
        .values()
        .max().unwrap_or(&0) {
        &1 => Some(passphrase_string),
        _  => None
    }
}