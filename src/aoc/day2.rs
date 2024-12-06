use std::fs::File;
use std::io::{self, BufRead};

pub fn call() {
    part1();
    part2();
}

fn get_file_data() -> Vec<Vec<usize>> {
    let file = File::open("src/aoc/data/day2.txt").unwrap();
    let reader = io::BufReader::new(file);
    reader
        .lines()
        .map(|line| {
            line.unwrap().split_whitespace()
                .map(|e| e.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn is_valid_line(line: &Vec<usize>) -> bool {
    line.windows(2)
        .all(|w| w[0].abs_diff(w[1]) > 0 && w[0].abs_diff(w[1]) < 4)
        && (line.is_sorted() || line.iter().rev().is_sorted())
}

fn part1() {
    let list = get_file_data();
    let res = list.iter().filter(|line| is_valid_line(&line)).count();
    println!("{:?}", res);
}

fn part2() {
    let list = get_file_data();
    let res = list
        .iter()
        .filter(|line| {
            is_valid_line(&line) || {
                (0..line.len()).any(|index| {
                    let mut new_list = line.to_vec();
                    new_list.remove(index);
                    is_valid_line(&new_list)
                })
            }
        })
        .count();
    println!("{:?}", res);
}
