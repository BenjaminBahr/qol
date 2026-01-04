pub mod read_lines;

use std::error::Error;
use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

pub fn read_lines<P: AsRef<Path>>(filename: &P) -> Vec<String> {
    read_to_string(filename)
        .unwrap_or_else(|err| {
            eprintln!("Error occurred when trying to read from {:?}: {}", filename.as_ref(), err);
            String::new() } ) // "Define exceptions out of existence!"
        .lines() // split the string into an iterator of string
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

pub fn read_lines_lazy<P: AsRef<Path>>(
    filename: P,
) -> Result<Lines<BufReader<File>>, Box<dyn Error>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_lines_returns_error_on_no_file() {
        // given
        let path: &str = "does_not_exist.txt";
        let expected_result: Vec<String> = vec![];

        // when
        let result = read_lines(&path);

        // then
        assert_eq!(result, expected_result);
    }

    #[test]
    fn read_lines_returns_lines_of_file() {
        // given
        let path: &str = "resources/file_for_test.txt";
        let expected_result = vec!["Line 1", "Line 2", "", "Line 3 Still line 4"];

        // when
        let result = read_lines(&path);

        // then
        assert_eq!(result, expected_result);
    }

    #[test]
    fn read_files_lazy_works() {
        // given
        let path: &Path = Path::new("resources/file_for_test.txt");

        // when
        let result = read_lines_lazy(path);

        // then
        assert!(result.is_ok());
        let lines = result.unwrap();
        let lines_as_string: Vec<String> =
            lines.filter_map(|line| line.ok()).collect::<Vec<String>>();
        assert_eq!(
            lines_as_string,
            vec!["Line 1", "Line 2", "", "Line 3 Still line 4"]
        );
    }
}
