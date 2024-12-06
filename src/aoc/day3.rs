use regex::{Captures, Regex};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

pub fn call() {
    get_file_data();
}

fn get_file_data() {
    let file = File::open("src/aoc/data/day3.txt").unwrap();
    let mut reader = io::BufReader::new(file);
    let mut data = String::new();

    reader.read_to_string(&mut data).unwrap();
    part1(&data);
    part2(&data);
}

fn part1(data: &str) {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let sum: usize = 0;
    let res: usize = regex.captures_iter(data).map(|word| get_mul(word)).sum();
    println!("{:?}", res);
}

fn part2(data: &str) {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don\'t\(\)").unwrap();
    let mut sum: usize = 0;
    let mut mul = true;
    regex.captures_iter(data).for_each(|word| {
        let int = word.get(0).unwrap().as_str();
        match int {
            "do()" => mul = true,
            "don't()" => mul = false,
            _ => {
                if mul {
                    sum += get_mul(word);
                }
            }
        }
    });
    println!("{:?}", sum);
}

fn get_mul(capture: Captures) -> usize {
    let l = capture[1].parse::<usize>().unwrap();
    let r = capture[2].parse::<usize>().unwrap();
    l * r
}
