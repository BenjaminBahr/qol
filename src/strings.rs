pub mod strip_of_non_digits_and_convert;

/// Examples:
/// ```
/// use crate::bb_qol::strings::strip_of_non_digits_and_convert;
/// let s1 = "I am 45 years old and have 0 regrets.";
/// let s2 = "I have 0 regrets and am 45 years old.";
/// let s3 = "-1";
/// let s4 = "";
///
/// assert_eq!(strip_of_non_digits_and_convert(s1), 450);
/// assert_eq!(strip_of_non_digits_and_convert(s2), 45);
/// assert_eq!(strip_of_non_digits_and_convert(s3), 1);
/// assert_eq!(strip_of_non_digits_and_convert(s4), 0);
/// ```
pub fn strip_of_non_digits_and_convert(s: &str) -> u32 {
    s.chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse()
        .unwrap_or_else(|err| {
            eprintln!("Could not parse {} to u32: {:?}", s, err);
            0
        })
}
