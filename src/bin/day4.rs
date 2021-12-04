use adventofcode::as_strings;
use adventofcode::read_file;
use std::collections::HashMap;

struct BingoCard {
    card_numbers: HashMap<String, (usize, usize)>,
    matched_numbers: Vec<Vec<bool>>,
}

impl BingoCard {
    fn new(input: Vec<String>) -> BingoCard {
        let parsed_board: Vec<Vec<String>> = input.iter().map(|s| s.split(" ").map(|n| String::from(n)).collect()).collect();

        let mut card_numbers = HashMap::new();
        for (row_n, card_row) in parsed_board.iter().enumerate() {
            for (col_n, card_number) in card_row.iter().enumerate() {
                card_numbers.insert(card_number.clone(), (row_n, col_n));
            }
        }

        let card_width = parsed_board[0].len();
        let card_height = parsed_board.len();
        BingoCard {
            card_numbers: card_numbers,
            matched_numbers: vec![vec![false; card_width]; card_height],
        }
    }

    fn try_add_number(&mut self, number: String) -> bool {
        match self.card_numbers.get(&number) {
            Some((row_n, col_n)) => {
                self.matched_numbers[*row_n][*col_n] = true;
                //now scan board and return if we won
                true
            }
            None => false,
        }
    }
}

fn part_1(filename: &str) -> usize {
    let lines = as_strings(read_file(filename));
    0
}

// fn part_2(filename: &str) -> usize{

// }

fn main() {
    assert_eq!(part_1("input/4_test.txt"), 4512);
    println!("Part 1: {:?}", part_1("input/4_input.txt"));
    // assert_eq!(part_2("input/4_test.txt"), );
    // println!("Part 2: {:?}", part_2("input/4_input.txt"));
}
