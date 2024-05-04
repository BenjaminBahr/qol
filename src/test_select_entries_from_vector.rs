use crate::select_entries_from_vector;

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
    }
    else {
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
    }
    else {
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
    }
    else {
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
