#[allow(dead_code)]
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

pub fn get_all_choices_from_range(range: usize) -> Vec<Vec<usize>> {
    let mut result = vec![];
    for i in 0..range + 1 {
        let next_choices: Vec<Vec<usize>> = choices_from_range(range, i);
        result.extend(next_choices);
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

    /*
    Scenario: range is smaller than the amount of selected entries.
    An empty array should be returned.
     */
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

    /*
    Scenario: Get all choices from range returns the correct amount of entries, including the maximum amount
     */
    #[test]
    fn test_get_all_entries() {

        // Given
        let range = 2;
        let expected_result: Vec<Vec<usize>> = vec![vec![], vec![0], vec![1], vec![0, 1], vec![1, 0]];

        // When
        let result = get_all_choices_from_range(range);

        // Then
        assert_eq!(result, expected_result);
    }
}
