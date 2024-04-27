use itertools::Itertools;

pub fn choices_from_range(range: usize, amount: usize) -> Vec<Vec<usize>> {
    // Generate all combinations of `n` elements from the range `0..r`
    let combinations = (0..range).combinations(amount);

    // For each combination, generate all permutations
    let mut result = Vec::new();
    for combination in combinations {
        let permutations = combination.into_iter().permutations(amount);
        result.extend(permutations);
    }

    result

}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_case_01() {
        // Given
        let range: usize = 3;
        let amount: usize = 2;
        let expected_result = vec![
            vec![0, 1],
            vec![0, 2],
            vec![1, 0],
            vec![2, 0],
            vec![1, 2],
            vec![2, 1],
        ];

        // When
        let result = choices_from_range(range, amount);

        // Then
        let mut set1 = std::collections::HashSet::new();
        let mut set2 = std::collections::HashSet::new();
        for vec in &result {
            set1.insert(vec.clone());
        }
        for vec in &expected_result {
            set2.insert(vec.clone());
        }
        assert_eq!(set1, set2);
    }

    #[test]
    fn test_case_02() {
        // Given
        let range: usize = 1;
        let amount: usize = 2;
        let expected_result: Vec<Vec<usize>> = vec![];

        // When
        let result = choices_from_range(range, amount);

        // Then
        assert_eq!(result, expected_result);

    }
}
