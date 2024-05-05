
pub fn strip_of_non_digits_and_convert(s: &str) -> u32 {
    let result: u32 = s
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse()
        .unwrap_or(0);
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {

        // Given
        let s = "I have 0 regrets. I am 43 years and 5 months old";
        let expected_result = 435;

        // When
        let result = strip_of_non_digits_and_convert(s);

        // Then
        assert_eq!(result, expected_result);
    }
}
