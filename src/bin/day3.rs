use adventofcode::as_strings;
use adventofcode::read_file;

fn invert_value(value: &String) -> String {
    value.chars().map(|c| if c == '0' {'1'} else {'0'}).collect()
}

fn part_1(filename: &str) -> isize {
    let lines = as_strings(read_file(filename));
    let mut gamma_rate = String::new();
    let mut bit_map: Vec<String> = vec![String::new(); lines[0].len()];
    for line in lines {
        for (index, character) in line.chars().enumerate() {
            bit_map[index].push(character);
        }
    }

    bit_map.iter().for_each(|v| {
        match v.matches('1').count() > v.chars().count() / 2 {
            true => gamma_rate.push('1'),
            false => gamma_rate.push('0')
        }
    });

    let epsilon_rate = invert_value(&gamma_rate);

    let gama_rate_as_int = isize::from_str_radix(gamma_rate.as_str(), 2).unwrap();
    let epsilon_rate_as_int = isize::from_str_radix(epsilon_rate.as_str(), 2).unwrap();

    gama_rate_as_int * epsilon_rate_as_int
}

fn main() {
    assert_eq!(part_1("input/3_test.txt"), 198);
    println!("Part 1: {:?}", part_1("input/3_input.txt"));
}
