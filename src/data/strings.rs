
pub fn strip_of_non_digits_and_convert(s: &str) -> u32 {
    let result: u32 = s
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse()
        .unwrap_or(0);
    return result;
}