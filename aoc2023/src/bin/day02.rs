use std::fs;
use std::time::Instant;
use std::cmp::max;

fn main() {
    let input = fs::read_to_string("data/day02.txt")
        .expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {:?}", part1_ans);

    let part2_start = Instant::now();
    let part2_ans = part2(&input);
    println!("Part 2 time: {:.2?}", part2_start.elapsed());
    println!("Part 2 ans: {:?}", part2_ans);
}

fn part1(input: &str) -> usize {
    input.split("\n").enumerate()
        .map(|(idx, line)| {
            let is_invalid = line.split(": ")
                .nth(1)
                .unwrap()
                .split("; ")
                .map(parse_set)
                .filter(|set| set.0 > 12 || set.1 > 13 || set.2 > 14)
                .count() > 0;

            (idx + 1, is_invalid)
        })
        .filter(|(_, is_invalid)| !is_invalid)
        .map(|(idx, _)| idx)
        .sum::<usize>()
}

fn part2(input: &str) -> usize {
    input.split("\n")
        .map(|line| {
            let cubes = line.split(": ")
                .nth(1)
                .unwrap()
                .split("; ")
                .map(parse_set)
                .fold((0, 0, 0), |acc, set| {
                    (max(acc.0, set.0), max(acc.1, set.1), max(acc.2, set.2))
                });
            cubes.0 * cubes.1 * cubes.2
        })
        .sum::<usize>()
}

fn parse_set(set: &str) -> (usize, usize, usize) {
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;

    for x in set.split(", ") {
        let mut parts = x.split(" ");
        let count = parts.next().unwrap().parse::<usize>().unwrap();
        let colour = parts.next().unwrap();

        match colour {
            "red" => r = count,
            "green" => g = count,
            "blue" => b = count,
            _ => unreachable!()
        }
    }

    (r, g, b)
}