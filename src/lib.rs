use std::fs::read_to_string;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod test_select_entry_from_vector;
mod test_select_entries_from_vector;

pub fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

pub fn strip_of_non_digits_and_convert(s: &str) -> u32 {
    let result: u32 = s
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse()
        .unwrap_or(0);
    return result;
}

pub fn have_same_sign(a: i32, b: i32) -> bool {
    return !((a < 0 && b > 0) || (a > 0 && b < 0));
}

pub fn select_entry_from_vector<T: Copy + Clone>(
    vector: &Vec<T>,
    entry: usize,
) -> (Option<T>, Vec<T>) {
    let mut vector_copy = vector.clone();
    if let Some(removed_entry) = vector.get(entry) {
        vector_copy.remove(entry);
        return (Some(*removed_entry), vector_copy);
    } else {
        return (None, vector_copy);
    }
}

pub fn select_entries_from_vector<T: Copy + Clone>(
    vec: Vec<T>,
    entries: Vec<usize>,
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
