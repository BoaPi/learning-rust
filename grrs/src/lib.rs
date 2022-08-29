use std::io::{BufRead, BufReader};

// function which receives a BufReader and patternn to look for in 
// each line of the BufReader
pub fn find_matches<R: BufRead>(reader: R, pattern: &str) -> String {
    // print each line of file which contains the pattern
    let mut check: String;
    let mut result: String = String::from("");

    for line in reader.lines() {
        check = line.unwrap();

        if check.contains(pattern) {
            result.push_str(&check);
            result.push_str("\n");
        }
    }

    return result;
} 

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_find_matches() {
        // arrange
        // generate byte stream from string
        // create new BufReader and pass source
        // create pattern string
        let source = "Lorem ipsum dolor\nipsum\ndolor some stuff".as_bytes();
        let reader = BufReader::new(source);
        let pattern: &str = "dolor";

        // act
        // call function to test
        // pass reader and pattern to function
        let result: String = find_matches(reader, pattern);

        // assert
        // create target match
        // compare result with target match
        let matches: String = String::from("Lorem ipsum dolor\ndolor some stuff\n");
        assert_eq!(result, matches);
    }
}
