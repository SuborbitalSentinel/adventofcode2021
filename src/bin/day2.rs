use adventofcode::read_file;
use adventofcode::tokenize_line;

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
    println!("Part 1: {:?}", part_1("input/2_input.txt"));
    println!("Part 2: {:?}", part_2("input/2_input.txt"));
}
