#![allow(dead_code)]

use std::fmt::Debug;
use std::fs;
use std::str::FromStr;

fn main() {
    // assert_eq!(day_0(), 0);
    // assert_eq!(day_1a(), 1602);
    // assert_eq!(day_1b(), 1633);
    // assert_eq!(day_2a(), 1499229);
    // assert_eq!(day_2b(), 1340836560);
    assert_eq!(day_3a(), 3895776);
}

fn numbers_to_vec<T>(filename: &str) -> Vec<T>
    where
        T: FromStr,
        T::Err: Debug,
{
    fs::read_to_string(filename)
        .expect("no file")
        .split_whitespace()
        .map(|s| {
            s.parse::<T>()
                .expect(format!("not a number {}", &s).as_str())
        })
        .collect()
}

fn lines_to_vec(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .expect("no file")
        .lines()
        .map(|s| s.to_owned())
        .collect()
}

fn day_0() -> i32 {
    println!("Hello World!");
    0
}

fn day_1a() -> i32 {
    let nums: Vec<i32> = numbers_to_vec("data/day_1.txt");

    let mut count = 0;
    for i in 1..nums.len() {
        if nums[i] > nums[i - 1] { count += 1; }
    }

    count
}

fn day_1b() -> i32 {
    let nums: Vec<i32> = numbers_to_vec("data/day_1.txt");

    let mut prev: i32 = nums.iter().take(3).sum();
    let mut count = 0;
    for i in 1..nums.len() - 2 {
        let cur = prev - nums[i - 1] + nums[i + 2];
        if cur > prev { count += 1; }
        prev = cur;
    }

    count
}

fn day_2a() -> i32 {
    let lines = lines_to_vec("data/day_2a.txt");
    let commands: Vec<(&str, i32)> = lines.iter()
        .map(|line| line.split_once(" ")).flatten()
        .map(|p| (p.0, p.1.parse::<i32>().unwrap())).collect();

    let result = commands.iter()
        .fold((0, 0), |(pos, depth), (dir, val)|
            match *dir {
                "forward" => (pos + val, depth),
                "down" => (pos, depth + val),
                "up" => (pos, depth - val),
                _ => panic!("unknown dir"),
            },
        );

    result.0 * result.1
}

fn day_2b() -> i32 {
    let lines = lines_to_vec("data/day_2a.txt");
    let commands: Vec<(&str, i32)> = lines.iter()
        .map(|line| line.split_once(" ")).flatten()
        .map(|p| (p.0, p.1.parse::<i32>().unwrap())).collect();

    let result = commands.iter()
        .fold((0, 0, 0), |(pos, depth, aim), (dir, val)|
            match *dir {
                "forward" => (pos + val, depth + (aim * val), aim),
                "down" => (pos, depth, aim + val),
                "up" => (pos, depth, aim - val),
                _ => panic!("unknown dir"),
            },
        );

    result.0 * result.1
}

fn day_3a() -> u32 {
    let lines = lines_to_vec("data/day_3a.txt");
    let rows = lines.len();
    let cols = lines[0].len();
    let mut counts: Vec<u32> = vec![0; cols];
    for line in &lines {
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                counts[i] += 1;
            }
        }
    }
    let half = (rows as u32) / 2;
    let gamma: u32 = counts.iter()
        .fold(0, |g, count| {
            if *count > half {
                (g << 1) + 1
            } else {
                g << 1
            }
        });
    let mask: u32 = (0..cols).fold(0, |m, _| (m << 1) + 1);
    let epsilon = (!gamma) & mask;

    gamma * epsilon
}




















