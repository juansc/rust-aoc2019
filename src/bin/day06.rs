use std::collections::HashMap;

use aoc2019::util;
use std::cmp::min;

fn main() {
    let lines = util::lines_from_file("./input/day06.txt");

    println!("Part 1 Solution: {}", part1(&lines));
    println!("Part 2 Solution: {}", part2(&lines));
}

fn get_orbits(lines: &[String]) -> HashMap<String, String> {
    let mut orbits = HashMap::new();
    lines
        .iter()
        .map(|l| l.split(')').collect::<Vec<&str>>())
        .for_each(|o| {
            orbits.insert(o[1].to_string(), o[0].to_string());
        });
    orbits
}

fn part1(lines: &[String]) -> u32 {
    // Create a mapping from satellite -> primary.
    let orbits = get_orbits(lines);

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

fn dist_to_com(body: String, orbits: &HashMap<String, String>) -> HashMap<String, u32> {
    let mut distances = HashMap::new();
    let mut body = orbits.get(&body).unwrap().clone();
    let mut counter = 0u32;
    while body != "COM" {
        counter += 1;
        body = orbits.get(&body).unwrap().clone();
        distances.insert(body.clone(), counter);
    }
    distances
}

fn part2(lines: &[String]) -> u32 {
    // Create a mapping from satellite -> primary.
    let orbits = get_orbits(lines);

    // Create a map indicating how many steps it took to go from YOU to the given body.
    // Create a map indicating how many steps it took to go from SAN to a given body.
    // Find the body that they have in common with the smallest commulative distance.
    let dist_to_you = dist_to_com("YOU".to_string(), &orbits);
    let dist_to_santa = dist_to_com("SAN".to_string(), &orbits);

    let mut min_distance = u32::MAX;
    for (body, d1) in dist_to_you.iter() {
        if let Some(d2) = dist_to_santa.get(body) {
            min_distance = min(d2 + d1, min_distance);
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
