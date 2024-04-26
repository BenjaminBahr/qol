#[allow(dead_code)]
use super::*;

#[test]
pub fn test_select_entry_from_vector() {
    let test_vector: Vec<i32> = vec![2, 3, 4];
    let (entry, vector) = select_entry_from_vector(&test_vector, 1);
    assert!(entry == Some(3));
    assert!(vector.starts_with(&[2]));
    assert!(vector.len() == 2);
}

#[test]
pub fn test_select_entry_from_vector_edge_case_01() {
    let test_vector: Vec<i32> = vec![2, 3, -1];
    let (entry, vector) = select_entry_from_vector(&test_vector, 5);
    assert!(entry == None);
    assert!(vector.len() == 3);
}

#[test]
pub fn test_select_entry_from_vector_edge_case_02() {
    let test_vector: Vec<i32> = vec![];
    let (entry, vector) = select_entry_from_vector(&test_vector, 0);
    assert!(entry == None);
    assert!(vector.len() == 0);
}
