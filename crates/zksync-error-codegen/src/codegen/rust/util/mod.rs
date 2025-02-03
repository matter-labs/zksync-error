pub mod codegen;

fn replace_non_alphanumeric(input: &str, replacement: char) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '<' || c == '>' {
                c
            } else {
                replacement
            }
        })
        .collect()
}
