pub fn sort(value: &String) -> String {
    let mut chars: Vec<char> = value.chars().collect();
    chars.sort();
    chars.into_iter().collect()
}