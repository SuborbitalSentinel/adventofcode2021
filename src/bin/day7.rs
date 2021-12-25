use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::ops::RangeInclusive;

pub fn atoi(input: &str) -> isize {
    input.parse::<isize>().unwrap()
}

fn part_1(filename: &str) -> isize {
    let mut line = String::new();
    let _ = BufReader::new(File::open(filename).expect("file not found")).read_line(&mut line);

    let initial_crabs: Vec<_> = line.trim_end().split(",").map(|v| atoi(v)).collect();
    let max_position = initial_crabs.iter().max().unwrap().clone();
    let number_of_crabs = initial_crabs.len();

    let mut possible_positions = vec![];
    RangeInclusive::new(0, max_position).for_each(|pos| possible_positions.push(vec![pos; number_of_crabs]));

    let mut minimum_edit_distance = isize::MAX;
    possible_positions.iter().for_each(|x| {
        let edit_distance: isize = initial_crabs.iter().zip(x).map(|(l, r)| (l - r).abs()).sum();
        if edit_distance < minimum_edit_distance {
            minimum_edit_distance = edit_distance;
        }
    });

    return minimum_edit_distance;
}

fn part_2(filename: &str) -> isize {
    let mut line = String::new();
    let _ = BufReader::new(File::open(filename).expect("file not found")).read_line(&mut line);

    let initial_crabs: Vec<_> = line.trim_end().split(",").map(|v| atoi(v)).collect();
    let max_position = initial_crabs.iter().max().unwrap().clone();
    let number_of_crabs = initial_crabs.len();

    let mut possible_positions = vec![];
    RangeInclusive::new(0, max_position).for_each(|pos| possible_positions.push(vec![pos; number_of_crabs]));
    let summation = |value: isize| -> isize { RangeInclusive::new(1, value).sum() };

    let mut minimum_edit_distance = isize::MAX;
    possible_positions.iter().for_each(|x| {
        let edit_distance: isize = initial_crabs.iter().zip(x).map(|(l, r)| summation((l - r).abs())).sum();
        if edit_distance < minimum_edit_distance {
            minimum_edit_distance = edit_distance;
        }
    });

    return minimum_edit_distance;
}

fn main() {
    assert_eq!(part_1("input/7_test.txt"), 37);
    println!("Part 1: {:?}", part_1("input/7_input.txt"));
    assert_eq!(part_2("input/7_test.txt"), 168);
    println!("Part 2: {:?}", part_2("input/7_input.txt"));
}
