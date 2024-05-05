use crate::vectors::select_entry_from_vector::select_entry_from_vector;

pub fn select_entries_from_vector<T: Clone>(
    vec: &Vec<T>,
    entries: &Vec<usize>,
) -> Option<(Vec<T>, Vec<T>)> {

    let mut entry_vector = entries.clone();
    let mut v1: Vec<T> = vec![];
    let mut v2: Vec<T> = vec.clone();

    for i in 0..entries.len() {
        let (returned_entry, reduced_array) = select_entry_from_vector(&v2, entry_vector[i]);
        if let Some(u) = returned_entry {
            v1.push(u);
            v2 = reduced_array;
            let ref_entry = entry_vector[i];
            for e in entry_vector.iter_mut() {
                if *e > ref_entry {
                    *e -= 1;
                }
            }

        } else {
            return None;
        }
    }
    Some((v1, v2))
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    pub fn test_entry_selection() {
        // Given
        let vec: Vec<i32> = vec![1, 2, 3, 4];
        let entries: Vec<usize> = vec![3, 1];

        // When
        let result = select_entries_from_vector(&vec, &entries);

        // Then
        if let Some((vec1, vec2)) = result {
            assert_eq!(vec1, vec![4, 2]);
            assert_eq!(vec2, vec![1, 3]);
        } else {
            panic!("{}", "Test returned None");
        }
    }

    #[test]
    pub fn test_entry_selection_02() {
        // Given
        let vec: Vec<i32> = vec![1, 2, 3, 4];
        let entries: Vec<usize> = vec![1, 3];

        // When
        let result = select_entries_from_vector(&vec, &entries);

        // Then
        if let Some((vec1, vec2)) = result {
            assert_eq!(vec1, vec![2, 4]);
            assert_eq!(vec2, vec![1, 3]);
        } else {
            panic!("{}", "Test returned None");
        }
    }

    #[test]
    pub fn test_entry_selection_03() {
        // Given
        let vec: Vec<i32> = vec![1, 2, 3, 4];
        let entries: Vec<usize> = vec![0, 1, 2, 3];

        // When
        let result = select_entries_from_vector(&vec, &entries);

        // Then
        if let Some((vec1, vec2)) = result {
            assert_eq!(vec1, vec![1, 2, 3, 4]);
            assert_eq!(vec2, vec![]);
        } else {
            panic!("{}", "Test returned None");
        }
    }

    #[test]
    pub fn test_entry_selection_04() {
        // Given
        let vec: Vec<i32> = vec![1, 2, 3, 4];
        let entries: Vec<usize> = vec![0, 1, 2, 3, 0];

        // When
        let result = select_entries_from_vector(&vec, &entries);

        // Then
        assert!(result == None);
    }
}
