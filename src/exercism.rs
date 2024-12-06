pub fn execute() {
    // nth prime
    // println!("{:?}", get_nth_prime(7));
    // println!("{:?}", nth_prime(7));

    // proverb
}

fn bitwise_fun() {
    let a = Allergies::new(255);

    println!("{:?}", a.check_all());
}

pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}
// pub enum Allergen {
//     Eggs = 1 << 0,
//     Peanuts = 1 << 1,
//     Shellfish = 1 << 2,
//     Strawberries = 1 << 3,
//     Tomatoes = 1 << 4,
//     Chocolate = 1 << 5,
//     Pollen = 1 << 6,
//     Cats = 1 << 7,
// }

use self::Allergen::*;
impl Allergies {
    pub fn new(score: u32) -> Self {
        let score = score % 256;
        Self { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        match allergen {
            Allergen::Eggs => self.score & 1 == 1,
            Allergen::Peanuts => self.score >> 1 & 1 == 1,
            Allergen::Shellfish => self.score >> 2 & 1 == 1,
            Allergen::Strawberries => self.score >> 3 & 1 == 1,
            Allergen::Tomatoes => self.score >> 4 & 1 == 1,
            Allergen::Chocolate => self.score >> 5 & 1 == 1,
            Allergen::Pollen => self.score >> 6 & 1 == 1,
            Allergen::Cats => self.score >> 7 & 1 == 1,
        }
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut all = Vec::new();
        let mut base = 0;
        while base <= 7 {
            let last_bit = self.score >> base;
            if last_bit & 1 == 1 {
                match base {
                    0 => all.push(Allergen::Eggs),
                    1 => all.push(Allergen::Peanuts),
                    2 => all.push(Allergen::Shellfish),
                    3 => all.push(Allergen::Strawberries),
                    4 => all.push(Allergen::Tomatoes),
                    5 => all.push(Allergen::Chocolate),
                    6 => all.push(Allergen::Pollen),
                    7 => all.push(Allergen::Cats),
                    _ => {}
                }
            }
            base += 1;
        }
        all
    }

    fn check(&self, allergen: &Allergen) -> bool {
        self.score & *allergen as u32 != 0
    }

    fn check_all(&self) -> Vec<Allergen> {
        ALLERGENS.into_iter().filter(|x| self.check1(x)).collect()
    }

    fn check1(&self, allergen: &Allergen) -> bool {
        self.score & (1 << *allergen as u8) != 0
    }

    fn check_all1(&self) -> Vec<Allergen> {
        (0..8u8)
            .map(|n| unsafe { std::mem::transmute(n) })
            .filter(|n| self.check1(n))
            .collect()
    }
}

const ALLERGENS: [Allergen; 8] = [
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
];

fn get_all_your_base() {
    let input = 10;
    let digits = &[4, 2];
    let output = 2;
    let a = all_your_base(digits, input, output);
    println!("{:?}", a);
}

fn all_your_base(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, BaseErr> {
    if to_base < 2 {
        return Err(BaseErr::OutputBase);
    }
    if from_base < 2 {
        return Err(BaseErr::InputBase);
    }

    let invalid_base = number.iter().find(|&n| *n >= from_base);
    if let Some(digit) = invalid_base {
        return Err(BaseErr::Digit(*digit));
    }

    if number.is_empty() {
        return Ok(vec![]);
    }

    let mut _value: u32 = number
        .iter()
        .rev()
        .enumerate()
        .map(|(i, n)| *n * (from_base).pow(i as u32))
        .sum();

    let normalized_num = number
        .iter()
        .copied()
        .fold(0, |acc, next| acc * from_base + next);

    let mut value = 0;
    for digit in number.iter() {
        if *digit >= from_base {
            return Err(BaseErr::Digit(*digit));
        }
        value = value * from_base + *digit;
    }

    let mut result = Vec::new();
    while value > 0 {
        // 42
        result.push(value % to_base);
        value /= to_base;
    }
    result.reverse();
    Ok(result)
}

enum BaseErr {
    InputBase,
    OutputBase,
    Digit(u32),
}

impl std::fmt::Debug for BaseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BaseErr::InputBase => write!(f, "InvalidInputBase"),
            BaseErr::OutputBase => write!(f, "InvalidOutputBase"),
            BaseErr::Digit(n) => write!(f, "InvalidDigit {n}"),
        }
    }
}

fn acronynm() {
    let a = "GNU Halley's _Comet_ namahNamah Namah-Prabhu";

    let ans = a
        .split_terminator(|c: char| c.is_ascii_whitespace() || c == '-')
        .filter(|w| !w.is_empty())
        .map(|w: &str| {
            let word = w[..1].to_ascii_uppercase() + &w[1..];
            if word == word.to_ascii_uppercase() {
                word[..1].to_string()
            } else {
                word
            }
        })
        .collect::<String>();
    let ans = ans
        .chars()
        .filter(|c| c.is_ascii_uppercase())
        .collect::<String>();

    let ans = a
        .split(|c: char| c.is_whitespace() || c == '-' || c == '_')
        .flat_map(|word| {
            word.chars().take(1).chain(
                word.chars()
                    .skip_while(|c| c.is_uppercase())
                    .filter(|c| c.is_uppercase()),
            )
        })
        .collect::<String>();
    println!("{:?}", ans);
}

fn kids_growing_plants() -> Vec<&'static str> {
    const N: usize = 1;
    let position = N * 2;

    let apply_char = |c: char| match c {
        'V' => "violets",
        'R' => "radishes",
        'C' => "clover",
        'G' => "grass",
        _ => "",
    };

    let a: Vec<_> = "KIDS\nAREE\nCool"
        .lines()
        .flat_map(|l| l.chars().skip(position * 2).take(2))
        .map(apply_char)
        .collect();

    "KIDS\nAREE\nCool"
        .lines()
        .flat_map(|line| line[position..=position + 1].chars().map(apply_char))
        .collect()
}
fn series_142() {
    println!("namah prabhu /\\ /\\");

    let a = "53224";
    let b: Vec<_> = a
        .chars()
        .collect::<Vec<char>>()
        .windows(3)
        .map(|x| x.iter().collect::<String>())
        .collect();
    let b: Vec<_> = a
        .as_bytes()
        .windows(3)
        .map(|x| String::from_utf8(x.to_vec()).unwrap())
        .collect();
    println!("{:?}", b);
}

// modular exponentiation
fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp >>= 1;
        base = base * base % modulus
    }
    result
}

fn private_key(p: u64) -> u64 {
    p
}
fn public_key(p: u64, g: u64, a: u64) -> u64 {
    g.pow(a as u32) % p
}
fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    b_pub.pow(a as u32) % p
}

fn get_nth_prime(nth: usize) -> usize {
    (2..).filter(|x| is_prime(*x)).nth(nth).unwrap()
}

fn is_prime(num: usize) -> bool {
    !(2..=(num as f64).sqrt() as usize).any(|i| num % i == 0)
}

fn nth_prime(n: usize) -> usize {
    let mut prime: Vec<usize> = Vec::new();
    (2..)
        .filter(|num| {
            if !prime.iter().any(|i| num % i == 0) {
                prime.push(*num);
                true
            } else {
                false
            }
        })
        .nth(n)
        .unwrap()
}

fn proverb(list: &[&str]) -> String {
    match list.first() {
        None => String::new(),
        Some(first_word) => list
            .windows(2)
            .map(|words| format!("{} {} prabhu\n", words[0], words[1]))
            .chain(std::iter::once(format!("{} prabhu /\\", first_word)))
            .collect(),
    }
}
