pub fn string_has_unique_chars(input: &str) -> bool {
    let mut buffer: Vec<char> = vec![];

    input.chars().all(|char| {
        if buffer.contains(&char) {
            return false;
        } else {
            buffer.push(char);
            return true;
        }
    })
}
