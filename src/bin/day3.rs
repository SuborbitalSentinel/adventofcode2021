use adventofcode::as_strings;
use adventofcode::invert_binary_string;
use adventofcode::read_file;

fn pivot(bit_map: &mut Vec<String>, line: &str) {
    line.chars().enumerate().for_each(|(i, c)| {
        bit_map[i].push(c);
    });
}

fn filter_group(current_group: &Vec<String>, current_index: usize, match_char: fn(&String) -> char) -> String {
    if current_group.len() == 1 {
        return current_group[0].clone();
    }

    let mut bit_map = vec![String::new(); current_group[0].len()];
    current_group.iter().for_each(|l| pivot(&mut bit_map, l));

    let winning_match = match_char(&bit_map[current_index]);

    let idxs_to_keep: Vec<_> = bit_map[current_index].match_indices(winning_match).map(|(idx, _)| idx).collect();
    let new_group = current_group.iter().enumerate().fold(vec![], |mut acc, (idx, value)| {
        if idxs_to_keep.contains(&idx) {
            acc.push(String::from(value))
        }
        acc
    });

    filter_group(&new_group, current_index + 1, match_char)
}

fn part_1(filename: &str) -> isize {
    let lines = as_strings(read_file(filename));
    let mut gamma_rate = String::new();
    let mut bit_map = vec![String::new(); lines[0].len()];
    lines.iter().for_each(|l| pivot(&mut bit_map, l));

    bit_map.iter().for_each(|v| match v.matches('1').count() > bit_map[0].len() / 2 {
        true => gamma_rate.push('1'),
        false => gamma_rate.push('0'),
    });

    let epsilon_rate = invert_binary_string(&gamma_rate);

    let gama_rate_as_int = isize::from_str_radix(gamma_rate.as_str(), 2).unwrap();
    let epsilon_rate_as_int = isize::from_str_radix(epsilon_rate.as_str(), 2).unwrap();

    gama_rate_as_int * epsilon_rate_as_int
}

fn part_2(filename: &str) -> isize {
    let lines = as_strings(read_file(filename));

    let oxygen_gen_rating = filter_group(&lines, 0, |s| if s.matches('1').count() >= s.matches('0').count() { '1' } else { '0' });
    let co2_scrubber_rating = filter_group(&lines, 0, |s| if s.matches('1').count() >= s.matches('0').count() { '0' } else { '1' });

    let oxygen_gen_rating_as_int = isize::from_str_radix(oxygen_gen_rating.as_str(), 2).unwrap();
    let co2_scrubber_rating_as_int = isize::from_str_radix(co2_scrubber_rating.as_str(), 2).unwrap();

    oxygen_gen_rating_as_int * co2_scrubber_rating_as_int
}

fn main() {
    assert_eq!(part_1("input/3_test.txt"), 198);
    println!("Part 1: {:?}", part_1("input/3_input.txt"));
    assert_eq!(part_2("input/3_test.txt"), 230);
    println!("Part 2: {:?}", part_2("input/3_input.txt"));
}
