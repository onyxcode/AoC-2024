use std::fs::read_to_string;

pub fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

pub fn has_duplicates<T: PartialEq>(slice: &[T]) -> bool {
    for i in 1..slice.len() {
        if slice[i..].contains(&slice[i - 1]) {
            return true;
        }
    }
    false
}