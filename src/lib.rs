use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;

pub fn read_file(filename: &str) -> Lines<BufReader<File>> {
    let file = File::open(filename).expect("file not found");
    BufReader::new(file).lines()
}

pub fn as_ints(file: Lines<BufReader<File>>) -> Vec<i32> {
    file.map(|l| l.expect("could not parse line"))
        .map(|l| l.parse::<i32>())
        .map(|n| n.expect("could not parse string to int"))
        .collect()
}

pub fn string_to_tuple(s: String) -> (String, i32) {
    let tokens: Vec<&str> = s.split(" ").collect();
    if tokens.len() != 2 {
        panic!("line didn't contain two tokens");
    }
    (String::from(tokens[0]), tokens[1].parse::<i32>().unwrap())
}

pub fn tokenize_line(file_lines: Lines<BufReader<File>>) -> Vec<(String, i32)> {
    file_lines
        .map(|l| l.unwrap())
        .map(|s| string_to_tuple(s))
        .collect()
}

