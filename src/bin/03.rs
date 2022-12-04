use itertools::Itertools;
use std::{collections::HashSet, time::Instant};

// Basically all stolen 🙂
fn main() {
    let input = std::fs::read_to_string("./src/inputs/03.txt").unwrap();

    // Part 1
    let start1 = Instant::now();
    let result1 = part1(input.clone());

    println!("Time Taken: {:?}", start1.elapsed());
    println!("{:?}", result1);

    // Part 2
    let start2 = Instant::now();
    let result2 = part2(input);

    println!("Time Taken: {:?}", start2.elapsed());
    println!("{:?}", result2);
}

fn part1(input: String) -> u32 {
    return input
        .lines()
        .map(|line| {
            let (left, right) = line.split_at(line.len() / 2);
            let left = left.chars().collect::<HashSet<_>>();
            let right = right.chars().collect::<HashSet<_>>();
            let mut common_char = left.intersection(&right);
            let common_char = *common_char.next().unwrap() as u8;

            if common_char.is_ascii_lowercase() {
                (common_char - b'a' + 1) as u32
            } else {
                (common_char - b'A' + 27) as u32
            }
        })
        .sum::<u32>();
}

fn part2(input: String) -> u32 {
    return input
        .lines()
        .tuples()
        .map(|(s1, s2, s3)| {
            let s1 = s1.chars().collect::<HashSet<_>>();
            let s2 = s2.chars().collect::<HashSet<_>>();
            let s3 = s3.chars().collect::<HashSet<_>>();

            let comm_1_2 = s1.intersection(&s2).copied().collect::<HashSet<_>>();
            let mut comm = comm_1_2.intersection(&s3);

            let comm = *comm.next().unwrap() as u8;

            if comm.is_ascii_lowercase() {
                (comm - b'a' + 1) as u32
            } else {
                (comm - b'A' + 27) as u32
            }
        })
        .sum::<u32>();
}
