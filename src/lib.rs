pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
pub mod test_select_entry_from_vector;
#[cfg(test)]
pub mod test_select_entries_from_vector;
mod data;
mod combinatorics;
mod arithmetics;

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



