use regex::{Captures, Regex};
use std::fs::File;
use std::io::{self, BufReader, Read};

pub fn call() {
    get_file_data();
}

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, 1),
    (-1, 0),
    (0, 1),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn get_file_data() {
    let file = File::open("src/aoc/data/day4.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut data = String::new();

    reader.read_to_string(&mut data).unwrap();
    part1(&parse_input(&data));
    part2(&parse_input(&data));
}

fn part1(grid: &Grid) {
    let mut total = 0;

    for y in 0..=grid.h {
        for x in 0..=grid.w {
            if grid.contents[y][x] != 'X' {
                continue;
            }
            for &(dx, dy) in DIRECTIONS.iter() {
                let end_x = (x as i32) + dx * 3;
                let end_y = (y as i32) + dy * 3;
                // Check if XMAS can fit
                if end_x < 0 || end_x > (grid.w as i32) || end_y < 0 || end_y > (grid.h as i32) {
                    continue;
                }

                let is_xmas = get_xmas_positions(x, y, dx, dy)
                    .into_iter()
                    .all(|(nx, ny, expected)| grid.contents[ny][nx] == expected);
                if is_xmas {
                    total += 1;
                }
            }
        }
    }
    println!("{:?}", total);
}

fn part2(grid: &Grid) {
    let mut total = 0;

    for y in 1..grid.h {
        for x in 1..grid.w {
            if grid.contents[y][x] != 'A' {
                continue;
            }
            let tl = grid.contents[y + 1][x - 1];
            let tr = grid.contents[y + 1][x + 1];
            let bl = grid.contents[y - 1][x - 1];
            let br = grid.contents[y - 1][x + 1];

            let a = tl == 'M' && tr == 'M' && br == 'S' && bl == 'S';
            let b = tl == 'S' && tr == 'M' && br == 'M' && bl == 'S';
            let c = tl == 'S' && tr == 'S' && br == 'M' && bl == 'M';
            let d = tl == 'M' && tr == 'S' && br == 'S' && bl == 'M';

            if a || b || c || d {
                total += 1;
            }
        }
    }
    println!("{:?}", total);
}

fn get_xmas_positions(x: usize, y: usize, dx: i32, dy: i32) -> [(usize, usize, char); 3] {
    let x_i32 = x as i32;
    let y_i32 = y as i32;
    [
        ((x_i32 + dx) as usize, (y_i32 + dy) as usize, 'M'),
        ((x_i32 + dx * 2) as usize, (y_i32 + dy * 2) as usize, 'A'),
        ((x_i32 + dx * 3) as usize, (y_i32 + dy * 3) as usize, 'S'),
    ]
}

struct Grid {
    w: usize,
    h: usize,
    contents: Vec<Vec<char>>,
}

fn parse_input(contents: &str) -> Grid {
    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let grid_height = grid.len() - 1;
    let grid_width = grid[0].len() - 1;

    Grid {
        w: grid_width,
        h: grid_height,
        contents: grid,
    }
}
