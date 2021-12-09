#![feature(drain_filter)]
use adventofcode::as_strings;
use adventofcode::read_file;
use std::collections::HashMap;
use std::collections::HashSet;

fn pivot(game_board: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut new_board = vec![Vec::new(); game_board[0].len()];
    game_board.iter().for_each(|l| {
        l.iter().enumerate().for_each(|(i, c)| {
            new_board[i].push(*c);
        });
    });
    new_board
}

struct BingoCard {
    number_to_pos: HashMap<String, (usize, usize)>,
    unmarked_card_numbers: HashSet<String>,
    matched_numbers: Vec<Vec<bool>>,
}

impl BingoCard {
    fn new(input: &Vec<String>) -> BingoCard {
        let parsed_board: Vec<Vec<String>> = input.iter().map(|s| s.split(" ").filter(|s| !s.is_empty()).map(|n| String::from(n)).collect()).collect();

        let mut card_numbers = HashSet::new();
        let mut number_to_pos = HashMap::new();
        for (row_n, card_row) in parsed_board.iter().enumerate() {
            for (col_n, card_number) in card_row.iter().enumerate() {
                card_numbers.insert(card_number.clone());
                number_to_pos.insert(card_number.clone(), (row_n, col_n));
            }
        }

        let card_width = parsed_board[0].len();
        let card_height = parsed_board.len();
        BingoCard {
            number_to_pos: number_to_pos,
            unmarked_card_numbers: card_numbers,
            matched_numbers: vec![vec![false; card_width]; card_height],
        }
    }

    fn mark_called_number(&mut self, number: &String) {
        match self.number_to_pos.get(number) {
            Some((row_n, col_n)) => {
                self.matched_numbers[*row_n][*col_n] = true;
                self.unmarked_card_numbers.remove(number);
            }
            None => {}
        }
    }

    fn check_rows(&self) -> bool {
        let winning_rows = self.matched_numbers.iter().enumerate().find(|(_, row)| row.iter().all(|x| *x));
        match winning_rows {
            Some((_, _)) => true,
            _ => false,
        }
    }

    fn check_cols(&self) -> bool {
        let pivoted_game_board = pivot(&self.matched_numbers);
        let winning_cols = pivoted_game_board.iter().enumerate().find(|(_, col)| col.iter().all(|x| *x));
        match winning_cols {
            Some((_, _)) => true,
            _ => false,
        }
    }

    fn sum_of_unmarked_numbers(&self) -> usize {
        self.unmarked_card_numbers.iter().map(|v| v.parse::<usize>().unwrap()).sum::<usize>()
    }

    fn has_bingo(&self) -> bool {
        match self.check_for_win() {
            Some(_) => true,
            None => false
        }
    }

    fn check_for_win(&self) -> Option<&BingoCard> {
        if self.check_rows() || self.check_cols() {
            return Some(self);
        }
        None
    }
}

fn parse_input(lines: Vec<String>) -> (Vec<String>, Vec<BingoCard>) {
    let game_input: Vec<String> = lines[0].split(',').map(|x| String::from(x)).collect();

    let mut game_boards = Vec::new();
    let mut current_board = Vec::new();
    for line in lines[2..].iter() {
        if line.is_empty() {
            game_boards.push(BingoCard::new(&current_board));
            current_board.clear();
            continue;
        }
        current_board.push(line.clone());
    }

    (game_input, game_boards)
}

fn part_1(filename: &str) -> usize {
    let (game_input, mut cards) = parse_input(as_strings(read_file(filename)));

    for number in game_input {
        cards.iter_mut().for_each(|b| b.mark_called_number(&number));
        let checked_cards: Vec<_> = cards.iter().map(|board| board.check_for_win()).flatten().collect();
        if checked_cards.len() > 0 {
            let winning_card = checked_cards[0];
            return number.parse::<usize>().unwrap() * winning_card.sum_of_unmarked_numbers();
        }
    }
    panic!("No one won!");
}

fn part_2(filename: &str) -> usize {
    let (game_input, mut remaining_cards) = parse_input(as_strings(read_file(filename)));

    for number in game_input {
        remaining_cards.iter_mut().for_each(|b| b.mark_called_number(&number));
        let winning_cards: Vec<_> = remaining_cards.drain_filter(|c| c.has_bingo()).collect();
        if remaining_cards.len() == 0 {
            let winning_card = &winning_cards[0];
            return number.parse::<usize>().unwrap() * winning_card.sum_of_unmarked_numbers();
        }
    }
    panic!("No one won!");
}

fn main() {
    assert_eq!(part_1("input/4_test.txt"), 4512);
    println!("Part 1: {:?}", part_1("input/4_input.txt"));
    assert_eq!(part_2("input/4_test.txt"), 1924);
    println!("Part 2: {:?}", part_2("input/4_input.txt"));
}
