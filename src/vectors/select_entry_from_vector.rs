
pub fn select_entry_from_vector<T: Clone>(
    vector: &Vec<T>,
    entry: usize,
) -> (Option<T>, Vec<T>) {
    let mut vector_copy = vector.clone();
    if let Some(removed_entry) = vector.get(entry) {
        vector_copy.remove(entry);
        return (Some(removed_entry.clone()), vector_copy);
    } else {
        return (None, vector_copy);
    }
}

#[cfg(test)]
mod tests {
    use crate::vectors::select_entry_from_vector::select_entry_from_vector;

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
}