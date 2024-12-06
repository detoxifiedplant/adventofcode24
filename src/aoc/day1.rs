pub fn call() {
    part1();
    part2();
}

fn get_file_data() -> (Vec<isize>, Vec<isize>){
    let (mut list1, mut list2): (Vec<_>, Vec<_>) = std::fs::read_to_string("data.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let mut words = line.split_whitespace();
            (
                words.next().unwrap().parse::<isize>().unwrap(),
                words.next().unwrap().parse::<isize>().unwrap(),
            )
        })
        .unzip();
    list1.sort();
    list2.sort();
    (list1, list2)
}

pub(crate) fn part1() {
    let (list1, list2) = get_file_data();
    let res: usize = list1
        .iter()
        .zip(list2.clone())
        .map(|(a, b)| a.abs_diff(b))
        .sum();
    println!("{:?}", res);
}

pub(crate) fn part2() {
    let (list1, list2) = get_file_data();
    let res = list1.iter().fold(0, |acc, e| {
        list2.iter().filter(|&x| x == e).count() as isize * e + acc
    });
    println!("{:?}", res);
}
