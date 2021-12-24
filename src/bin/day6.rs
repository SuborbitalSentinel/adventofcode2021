use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn atoi(input: &str) -> usize {
    input.parse::<usize>().unwrap()
}

fn part_1(filename: &str) -> usize {
    let mut line = String::new();
    let _ = BufReader::new(File::open(filename).expect("file not found")).read_line(&mut line);

    let initial_fish: Vec<_> = line.trim_end().split(",").map(|v| atoi(v)).collect();
    let mut fish_states: [usize; 9] = [0; 9];
    initial_fish.iter().for_each(|f| fish_states[*f] += 1);

    for _ in 0..80 {
        let new_fish = fish_states[0];
        fish_states.rotate_left(1);
        fish_states[6] += fish_states[8];
        fish_states[8] = new_fish;
    }

    fish_states.iter().sum()
}

fn part_2(filename: &str) -> usize {
    let mut line = String::new();
    let _ = BufReader::new(File::open(filename).expect("file not found")).read_line(&mut line);

    let initial_fish: Vec<_> = line.trim_end().split(",").map(|v| atoi(v)).collect();
    let mut fish_states: [usize; 9] = [0; 9];
    initial_fish.iter().for_each(|f| fish_states[*f] += 1);

    for _ in 0..256 {
        let new_fish = fish_states[0];
        fish_states.rotate_left(1);
        fish_states[6] += fish_states[8];
        fish_states[8] = new_fish;
    }

    fish_states.iter().sum()
}

fn main() {
    assert_eq!(part_1("input/6_test.txt"), 5934);
    println!("Part 1: {:?}", part_1("input/6_input.txt"));
    assert_eq!(part_2("input/6_test.txt"), 26984457539);
    println!("Part 2: {:?}", part_2("input/6_input.txt"));
}
