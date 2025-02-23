pub(crate) fn pretty_print_fragment(text: &str, line: usize, column: usize) -> String {
    let half_window = 3;
    let first_line = (line - half_window).max(0);
    let last_line_excl = (line + half_window).min(text.lines().count());

    let mut result = String::with_capacity(1024);
    for (text_line, line_no) in text
        .lines()
        .skip(first_line)
        .take(last_line_excl - first_line)
        .zip(first_line + 1..)
    {
        result.push_str(&format!("{line_no:6}{text_line}\n"));
        if line_no == line {
            for _ in 0..column + 6 {
                result.push(' ');
            }
            result.push_str("^\n");
        }
    }
    result
}

#[allow(unused)]
pub(crate) fn vec_display<T>(vec: &Vec<T>, sep: &str) -> String
where
    T: std::fmt::Display,
{
    vec.iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(sep)
}

pub(crate) fn vec_debug<T>(vec: &Vec<T>, sep: &str) -> String
where
    T: std::fmt::Debug,
{
    vec.iter()
        .map(|x| format!("{x:#?}"))
        .collect::<Vec<_>>()
        .join(sep)
}
