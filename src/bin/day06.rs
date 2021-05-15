use std::collections::HashMap;

use aoc2019::util;
use std::borrow::Borrow;
use std::cmp::min;

fn main() {
    let lines = util::lines_from_file("./input/day04.txt");

    println!("Part 1 Solution: {}", part1(&lines));
    println!("Part 2 Solution: {}", part2(&lines));
}

fn part1(lines: &[String]) -> u32 {
    // Create a mapping from satellite -> primary.
    let mut orbits = HashMap::new();
    lines
        .iter()
        .map(|l| l.split(')').collect::<Vec<&str>>())
        .for_each(|o| {
            orbits.insert(o[1].to_string(), o[0].to_string());
        });

    // For every body, repeatedly go down the primary until you get to the COM.
    let mut counter = 0;
    for body in orbits.keys() {
        let mut key = body.to_string();
        while key != "COM" {
            counter += 1;
            key = orbits.get(&key).unwrap().clone();
        }
    }
    counter
}

fn part2(lines: &[String]) -> u32 {
    // Create a mapping from satellite -> primary.
    let mut orbits = HashMap::new();
    lines
        .iter()
        .map(|l| l.split(')').collect::<Vec<&str>>())
        .for_each(|o| {
            orbits.insert(o[1].to_string(), o[0].to_string());
        });

    // Create a map indicating how many steps it took to go from YOU to the given body.
    // Create a map indicating how many steps it took to go from SAN to a given body.
    // Find the body that they have in common with the smallest commulative distance.
    let mut dist_to_you = HashMap::new();
    let mut body = orbits.get(&"YOU".to_string()).unwrap().clone();
    let mut counter = 0u32;
    while body != "COM" {
        counter += 1;
        body = orbits.get(&body).unwrap().clone();
        dist_to_you.insert(body.clone(), counter);
    }

    let mut dist_to_santa = HashMap::new();
    let mut body = orbits.get(&"SAN".to_string()).unwrap().clone();
    let mut counter = 0u32;
    while body != "COM" {
        counter += 1;
        body = orbits.get(&body).unwrap().clone();
        dist_to_santa.insert(body.clone(), counter);
    }
    let mut min_distance = u32::MAX;
    for (body, dist) in dist_to_you.iter() {
        match dist_to_santa.get(body) {
            Some(d) => {
                let dd = d + dist;
                if dd < min_distance {
                    min_distance = dd;
                }
            }
            _ => {}
        }
    }
    min_distance
}

#[cfg(test)]
mod tests {
    use aoc2019::util;

    use crate::{part1, part2};

    #[test]
    fn test_part1() {
        let input = util::lines_from_file("./input/day06.txt");
        assert_eq!(part1(&input), 110190)
    }

    #[test]
    fn test_part2() {
        let input = util::lines_from_file("./input/day06.txt");
        assert_eq!(part2(&input), 343)
    }
}
