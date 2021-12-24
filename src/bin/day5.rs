use adventofcode::as_strings;
use adventofcode::atoi;
use adventofcode::read_file;
use std::collections::HashMap;
use std::ops::RangeInclusive;

type GameBoard = HashMap<(isize, isize), usize>;

#[derive(Debug)]
struct Line {
    start: (isize, isize),
    end: (isize, isize),
}

impl Line {
    fn add_to_board(&self, board: &mut GameBoard) {
        for (x, y) in self.iterate() {
            *board.entry((x, y)).or_insert(0) += 1;
        }
    }

    fn is_horizontal(&self) -> bool {
        self.start.1 == self.end.1
    }

    fn is_vertical(&self) -> bool {
        self.start.0 == self.end.0
    }

    fn iterate(&self) -> Vec<(isize, isize)> {
        if self.is_vertical() {
            let x = self.start.0;
            let (start_y, end_y) = if self.start.1 <= self.end.1 { (self.start.1, self.end.1) } else { (self.end.1, self.start.1) };
            return RangeInclusive::new(start_y, end_y).map(|y| (x, y)).collect();
        } else if self.is_horizontal() {
            let y = self.start.1;
            let (start_x, end_x) = if self.start.0 <= self.end.0 { (self.start.0, self.end.0) } else { (self.end.0, self.start.0) };
            return RangeInclusive::new(start_x, end_x).map(|x| (x, y)).collect();
        } else {
            let x_mod: isize = if self.start.0 < self.end.0 { 1 } else { -1 };
            let y_mod: isize = if self.start.1 < self.end.1 { 1 } else { -1 };
            let mut points = vec![];
            let mut current = self.start;

            while current != self.end {
                points.push(current);
                current = (current.0 + x_mod, current.1 + y_mod);
            }
            points.push(self.end);

            return points;
        }
    }
}

fn parse_input(input: Vec<String>) -> Vec<Line> {
    let mut output = vec![];
    for line in input {
        let (start, end) = line
            .split_once(" -> ")
            .map(|(s, e)| (s.split_once(",").unwrap(), e.split_once(",").unwrap()))
            .map(|((sx, sy), (ex, ey))| ((atoi(sx), atoi(sy)), (atoi(ex), atoi(ey))))
            .unwrap();
        output.push(Line { start: start, end: end });
    }
    output
}

fn part_1(filename: &str) -> usize {
    let all_lines = parse_input(as_strings(read_file(filename)));
    let lines: Vec<_> = all_lines.iter().filter(|l| l.is_horizontal() || l.is_vertical()).collect();
    let mut board = GameBoard::new();
    lines.iter().for_each(|line| line.add_to_board(&mut board));
    board.values().map(|overlaps| if overlaps > &1 { 1 } else { 0 }).sum::<usize>()
}

fn part_2(filename: &str) -> usize {
    let lines = parse_input(as_strings(read_file(filename)));
    let mut board = GameBoard::new();
    lines.iter().for_each(|line| line.add_to_board(&mut board));
    board.values().map(|overlaps| if overlaps > &1 { 1 } else { 0 }).sum()
}

fn main() {
    assert_eq!(part_1("input/5_test.txt"), 5);
    println!("Part 1: {:?}", part_1("input/5_input.txt"));
    assert_eq!(part_2("input/5_test.txt"), 12);
    println!("Part 2: {:?}", part_2("input/5_input.txt"));
}
