// 199 -- 0: 0
// 200 -- 1: 0 1
// 208 -- 2: 0 1 2
// 210 -- 3: 1 2 3
// 200 -- 4: 4 2 3
// 207 -- 5: 4 5 3
// 240 -- 6: 4 5 6
// 269 -- 7: 5 6 7
// 260 -- 8: 6 7
// 263 -- 9: 7
// 0 + 1 + 2 < 1 + 2 + 3 => 0 < 3
// 1 + 2 + 3 < 2 + 3 + 4 => 1 < 4

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;

fn read_file(filename: &str) -> Lines<BufReader<File>> {
    let file = File::open(filename).expect("file not found");
    BufReader::new(file).lines()
}

fn as_ints(file: Lines<BufReader<File>>) -> Vec<i32> {
    file.map(|l| l.expect("could not parse line"))
        .map(|l| l.parse::<i32>())
        .map(|n| n.expect("could not parse string to int"))
        .collect()
}

fn part_1() -> i32 {
    let lines = as_ints(read_file("input.txt"));
    let mut number_of_increases = 0;
    let mut previous_number = lines[0];
    for number in &lines[1..] {
        if number > &previous_number {
            number_of_increases += 1;
        }
        previous_number = *number;
    }

    number_of_increases
}

fn part_2() -> i32 {
    let lines = as_ints(read_file("input.txt"));
    let max_lines = lines.len();
    let mut number_of_increases = 0;
    for (i, number) in lines.iter().enumerate() {
        if (i + 3) < max_lines && number < &lines[i + 3] {
            number_of_increases += 1;
        }
    }
    number_of_increases
}

fn main() {
    println!("Number of increases part1: {:?}", part_1());
    println!("Number of increases part2: {:?}", part_2());
}
