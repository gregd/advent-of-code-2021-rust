#![allow(dead_code)]

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;
use std::str::FromStr;

fn main() {
    // assert_eq!(day_0(), 0);
    // assert_eq!(day_1a(), 1602);
    // assert_eq!(day_1b(), 1633);
    // assert_eq!(day_2a(), 1499229);
    // assert_eq!(day_2b(), 1340836560);
    // assert_eq!(day_3a(), 3895776);
    // assert_eq!(day_3b(), 7928162);
    assert_eq!(day_4a(), 87456);
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

fn day_3b() -> u32 {
    let lines = lines_to_vec("data/day_3a.txt");

    fn find_elem<'a, 'b>(char1: char, char2: char, pos: usize, lines: &'a Vec<&'b str>) -> &'b str {
        if lines.len() == 1 { return lines[0]; }
        let mut counts: Vec<u32> = vec![0; lines[0].len()];
        for line in lines {
            for (i, c) in line.chars().enumerate() {
                if c == '1' {
                    counts[i] += 1;
                }
            }
        }
        let ones = counts[pos];
        let zeros = (lines.len() as u32) - ones;
        let keep = match ones.cmp(&zeros) {
            Ordering::Greater => char1,
            Ordering::Equal => char1,
            Ordering::Less => char2,
        };
        let new_lines = lines.iter().filter(|line| line.chars().nth(pos).unwrap() == keep).map(|i| *i).collect();
        find_elem(char1, char2, pos + 1, &new_lines)
    }

    fn convert_bits(s: &str) -> u32 {
        s.chars().fold(0, |cur, bit| (cur << 1) + if bit == '1' { 1 } else { 0 })
    }

    let pom = lines.iter().map(|line| line.as_str()).collect();
    let oxygen = convert_bits(find_elem('1', '0', 0, &pom));
    let co2 = convert_bits(find_elem('0', '1', 0, &pom));

    oxygen * co2
}

fn day_4a() -> i32 {
    let lines = lines_to_vec("data/day_4a.txt");
    let nums = 5;
    let draw_numbers: Vec<_> = lines[0].trim().split(',').map(|s| s.parse::<i32>()).flatten().collect();

    let mut boards = vec![];
    for chunk in lines.iter().skip(2).collect::<Vec<_>>().chunks(nums + 1) {
        let board: Vec<_> = chunk.iter()
            .filter(|s| s.len() > 0)
            .map(|s| s.split_ascii_whitespace().map(|n| n.parse::<i32>()).flatten().collect::<Vec<_>>())
            .collect();
        boards.push(board);
    }

    struct Point {
        board: usize,
        row: usize,
        col: usize,
    }

    let mut points: HashMap<i32, Vec<Point>> = HashMap::new();
    for (bnr, board) in boards.iter().enumerate() {
        for (rnr, row) in board.iter().enumerate() {
            for (cnr, &val) in row.iter().enumerate() {
                let pvec = points.entry(val).or_insert(vec![]);
                pvec.push(Point { board: bnr, row: rnr, col: cnr });
            }
        }
    }

    for cur in draw_numbers {
        if let Some(pvec) = points.get(&cur) {
            for point in pvec {
                let board = &mut boards[point.board];
                let row = &mut board[point.row];
                row[point.col] = -1;
                if row.iter().filter(|&&n| n >= 0).count() == 0 ||
                    (0..nums).map(|r| board[r][point.col]).filter(|&n| n >= 0).count() == 0
                {
                    return cur * board.iter().flatten().filter(|&&n| n > 0).sum::<i32>();
                }
            }
        }
    }

    0
}
















