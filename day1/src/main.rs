use std::collections::HashMap;
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

    return number_of_increases;
}

// fn part_2() -> i32 {
//     let lines = as_ints(read_file("test.txt"));
//     let mut groups = HashMap::new();
//     for (i, number) in lines.iter().enumerate() {
//         *groups.entry(i/3).or_insert(*number) += number;
//     }

//     let mut number_of_increases = 0;
//     let mut previous_number = groups[&0];
//     for number in groups.values().skip(1) {
//         if number > &previous_number {
//             number_of_increases += 1;
//         }
//         previous_number = *number;
//     }

//     return number_of_increases;
// }

fn main() {
    println!("Number of increases part1: {:?}", part_1());
    // println!("Number of increases part1: {:?}", part_2());
}
