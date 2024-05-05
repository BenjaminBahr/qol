
pub fn have_same_sign(a: i32, b: i32) -> bool {
    ( a > 0 && b > 0 )
    || ( a < 0 && b < 0 )
    || ( a == 0 && b == 0)
}

#[cfg(test)]
mod tests {
    use crate::arithmetics::have_same_sign::have_same_sign;

    #[test]
    fn test_same_sign() {
        // Given
        let a = 3;
        let b = 4;
        let c = -4;
        let d = -9;

        // When
        let result1 = have_same_sign(a, b);
        let result2 = have_same_sign(c, d);

        // Then
        assert_eq!(result1, true);
        assert_eq!(result2, true);
    }

    #[test]
    fn test_different_signs() {
        // Given
        let a = 3;
        let b = -4;
        let c = -4;
        let d = 6;

        // When
        let result1 = have_same_sign(a, b);
        let result2 = have_same_sign(c, d);

        // Then
        assert_eq!(result1, false);
        assert_eq!(result2, false);
    }

    #[test]
    fn test_one_zero() {
        // Given
        let a = 3;
        let b = -4;

        // When
        let result1 = have_same_sign(a, 0);
        let result2 = have_same_sign(b, 0);
        let result3 = have_same_sign( 0, a);
        let result4 = have_same_sign( 0, b);

        // Then
        assert_eq!(result1, false);
        assert_eq!(result2, false);
        assert_eq!(result3, false);
        assert_eq!(result4, false);
    }
}
