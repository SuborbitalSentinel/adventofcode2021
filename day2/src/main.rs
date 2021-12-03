use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;

fn read_file(filename: &str) -> Lines<BufReader<File>> {
    let file = File::open(filename).expect("file not found");
    BufReader::new(file).lines()
}

fn string_to_tuple(s: String) -> (String, i32) {
    let tokens: Vec<&str> = s.split(" ").collect();
    if tokens.len() != 2 {
        panic!("line didn't contain two tokens");
    }
    (String::from(tokens[0]), tokens[1].parse::<i32>().unwrap())
}

fn tokenize_line(file_lines: Lines<BufReader<File>>) -> Vec<(String, i32)> {
    file_lines
        .map(|l| l.unwrap())
        .map(|s| string_to_tuple(s))
        .collect()
}

fn part_1(filename: &str) -> i32 {
    let lines = tokenize_line(read_file(filename));

    let mut horizontal_position = 0;
    let mut vertical_position = 0;
    for (direction, magnitude) in lines {
        match direction.as_str() {
            "forward" => horizontal_position += magnitude,
            "down" => vertical_position += magnitude,
            "up" => vertical_position -= magnitude,
            _ => unreachable!(),
        }
    }
    horizontal_position * vertical_position
}

fn part_2(filename: &str) -> i32 {
    let lines = tokenize_line(read_file(filename));

    let mut horizontal_position = 0;
    let mut vertical_position = 0;
    let mut aim = 0;
    for (direction, magnitude) in lines {
        match direction.as_str() {
            "forward" => {
                horizontal_position += magnitude;
                vertical_position += aim * magnitude;
            },
            "down" => aim += magnitude,
            "up" => aim -= magnitude,
            _ => unreachable!(),
        }
    }
    horizontal_position * vertical_position
}

fn main() {
    println!("Part 1: {:?}", part_1("input.txt"));
    println!("Part 2: {:?}", part_2("input.txt"));
}
