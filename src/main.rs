#![allow(dead_code)]

use std::fmt::Debug;
use std::fs;
use std::str::FromStr;

fn main() {
    // assert_eq!(day_0(), 0);
    assert_eq!(day_1a(), 1602);
    assert_eq!(day_1b(), 1633);
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
    let nums: Vec<_> = lines_to_vec("data/day_1.txt")
        .iter().map(|s| s.parse::<i32>().unwrap()).collect();

    let mut count = 0;
    for i in 1..nums.len() {
        if nums[i] > nums[i - 1] { count += 1; }
    }

    count
}

fn day_1b() -> i32 {
    let nums: Vec<_> = lines_to_vec("data/day_1.txt")
        .iter().map(|s| s.parse::<i32>().unwrap()).collect();

    let mut prev: i32 = nums.iter().take(3).sum();
    let mut count = 0;
    for i in 1..nums.len() - 2 {
        let cur = prev - nums[i - 1] + nums[i + 2];
        if cur > prev { count += 1; }
        prev = cur;
    }

    count
}



