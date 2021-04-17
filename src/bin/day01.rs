use aoc2019::util;

/// Fuel is calculated using integer arithmetic: max(mass / 3 - 2, 0).
fn fuel_for_mass(mass: usize) -> usize {
    const OFFSET: usize = 2;
    let fuel = mass / 3;
    // Can't have negative fuel
    if fuel <= OFFSET {
        return 0;
    }
    fuel - OFFSET
}

/// Returns the fuel required for both the given mass as well as the fuel needed for the fuel itself.
fn fuel_for_payload(payload: usize) -> usize {
    let fuel_mass = fuel_for_mass(payload);
    // Note that if the given mass requires no fuel we don't need to calculate the mass for the fuel.
    if fuel_mass == 0 {
        return fuel_mass;
    }
    fuel_mass + fuel_for_payload(fuel_mass)
}

/// collapse takes in an array of strings, evaluates the mass_fn for each line, and then returns
/// the sum of the results.
fn collapse(lines: &[String], mass_fn: fn(usize) -> usize) -> usize {
    lines
        // Iterate over all the lines
        .iter()
        // Map each entry to a number using parse
        .map(|line| line.parse::<usize>())
        // Find all elements that are errors and log it. This lets you intercept
        // the results and do something and pass it on
        .inspect(|n| {
            if let Err(ref e) = *n {
                println!("Parsing error: {}", e)
            }
        })
        // Only keep successful parses
        .filter_map(Result::ok)
        // Now apply the mass function
        .map(|n| mass_fn(n))
        // sum. Note that we exclude a ; so this whole statement is an expression
        .sum()
}

pub fn part1(lines: &[String]) -> usize {
    collapse(lines, fuel_for_mass)
}

pub fn part2(lines: &[String]) -> usize {
    collapse(lines, fuel_for_payload)
}

fn main() {
    let lines = util::lines_from_file("./input/day01.txt");
    println!("Solution for Part 1: {}", part1(lines.as_slice()));
    println!("Solution for Part 2: {}", part2(lines.as_slice()));
}

// Tests

#[cfg(test)]
mod tests {
    use crate::{fuel_for_mass, fuel_for_payload, part1, part2};
    use crate::util;

    #[test]
    fn test_fuel_for_mass() {
        assert_eq!(fuel_for_mass(12), 2);
        assert_eq!(fuel_for_mass(14), 2);
        assert_eq!(fuel_for_mass(1969), 654);
        assert_eq!(fuel_for_mass(100756), 33583);
    }

    #[test]
    fn test_fuel_for_payload() {
        assert_eq!(fuel_for_payload(12), 2);
        assert_eq!(fuel_for_payload(1969), 966);
        assert_eq!(fuel_for_payload(100756), 50346);
    }

    #[test]
    fn test_part1() {
        let line = &[String::from("12")];
        assert_eq!(part1(line), 2);
        let lines = util::lines_from_file("./input/day01.txt");
        let lines = lines.as_slice();
        assert_eq!(part1(lines), 3405637);
    }

    #[test]
    fn test_part2() {
        let line = &[String::from("12")];
        assert_eq!(part2(line), 2);
        let lines = util::lines_from_file("./input/day01.txt");
        let lines = lines.as_slice();
        assert_eq!(part2(lines), 5105597);
    }
}