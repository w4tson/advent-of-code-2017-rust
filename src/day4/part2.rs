pub fn ordered_chars<S : Into<String>>(s : S) -> String {
    let mut chars: Vec<char> = s.into().chars().collect();
    chars.sort();
    chars.into_iter().collect()
}

pub fn sort_chars_in_words<S : Into<String>>(s : S) -> String {
    let words : Vec<String> = s.into()
        .split_whitespace()
        .map(ordered_chars)
        .collect();
        
    words.join(" ")
}