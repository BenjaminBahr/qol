/// Examples:
/// ```
/// use crate::bb_qol::arithmetics::partial_sum::partial_sum;
///
/// let vector_of_numbers: Vec<i32> = vec![1, 2, 0, -4, -5];
/// let partial_sums = partial_sum(&vector_of_numbers);
///
/// assert_eq!(partial_sums, vec![1, 3, 3, -1, -6]);
/// ```
pub fn partial_sum(n: &Vec<i32>) -> Vec<i32> {
    n.iter()
        .scan(0, |sum, &x| {
            *sum += x; // Update the running total
            Some(*sum) // Yield the current total
        })
        .collect()
}
