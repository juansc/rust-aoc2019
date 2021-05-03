use aoc2019::util;

fn main() {
    let lines = util::lines_from_file("./input/day04.txt");
    println!("Part 1 Solution: {}", part1(&lines));
    println!("Part 2 Solution: {}", part2(&lines));
}

fn part1(lines: &[String]) -> u32 {
    let bounds = lines[0].split('-').collect::<Vec<&str>>();
    let lower_bound = bounds[0].parse::<u32>().unwrap();
    let upper_bound = bounds[1].parse::<u32>().unwrap();
    (lower_bound..upper_bound + 1)
        // Count all instances where the password is valid
        .map(|n| is_valid_part1(n) as u32)
        .sum()
}

fn part2(lines: &[String]) -> u32 {
    let bounds = lines[0].split('-').collect::<Vec<&str>>();
    let lower_bound = bounds[0].parse::<u32>().unwrap();
    let upper_bound = bounds[1].parse::<u32>().unwrap();
    (lower_bound..upper_bound + 1)
        // Count all instances where the password is valid
        .map(|n| is_valid_part2(n) as u32)
        .sum()
}

fn is_valid_part1(password: u32) -> bool {
    vec![
        password_valid_range(password),
        password_monotonically_incr(password),
        has_max_consecutive_digits(password, 6),
    ]
    .iter()
    .all(|x| *x)
}

fn is_valid_part2(password: u32) -> bool {
    vec![
        password_valid_range(password),
        password_monotonically_incr(password),
        has_max_consecutive_digits(password, 2),
    ]
    .iter()
    .all(|x| *x)
}

fn password_valid_range(password: u32) -> bool {
    // Password must be six digits
    (100_000..1_000_000).contains(&password)
}

fn password_monotonically_incr(mut password: u32) -> bool {
    let mut last_digit = u32::MAX;
    let mut current_digit: u32;
    loop {
        current_digit = password % 10;
        if current_digit > last_digit {
            return false;
        }
        // Trim the least significant digit
        last_digit = current_digit;
        password /= 10;
        // No more digits left
        if password == 0 {
            break;
        }
    }
    true
}

// Checks that the password has a streak that has at most `limit` of the same
// digit in a row.
fn has_max_consecutive_digits(mut password: u32, limit: u32) -> bool {
    let mut last_digit = u32::MAX;
    let mut current_digit: u32;
    let mut streak_length = 0u32;
    loop {
        current_digit = password % 10;
        if current_digit == last_digit {
            streak_length += 1;
        } else {
            // The streak is over. If the streak length is up to limit long, return true
            if (2..limit + 1).contains(&streak_length) {
                return true;
            }
            streak_length = 1;
        }
        // Trim the least significant digit
        last_digit = current_digit;
        password /= 10;
        // No more digits left
        if password == 0 {
            break;
        }
    }
    (2..limit + 1).contains(&streak_length)
}

#[cfg(test)]
mod tests {
    use aoc2019::util;

    use crate::{has_max_consecutive_digits, is_valid_part1, is_valid_part2, part1, part2};

    #[test]
    fn test_password_part1() {
        assert_eq!(is_valid_part1(111111), true);
        assert_eq!(is_valid_part1(112233), true);
        assert_eq!(is_valid_part1(145677), true);
        assert_eq!(is_valid_part1(778899), true);
        // Digits not monotonically increasing
        assert_eq!(is_valid_part1(101111), false);
        // Number is too small
        assert_eq!(is_valid_part1(10), false);
        assert_eq!(is_valid_part1(99999), false);
        // Number is too big
        assert_eq!(is_valid_part1(1111111), false);
        // No adjacent numbers that are the same
        assert_eq!(is_valid_part1(123456), false);
    }

    #[test]
    fn test_password_part2() {
        assert_eq!(is_valid_part2(111111), false);
        assert_eq!(is_valid_part2(112233), true);
        assert_eq!(is_valid_part2(145677), true);
        assert_eq!(is_valid_part2(778899), true);
        // Digits not monotonically increasing
        assert_eq!(is_valid_part2(101111), false);
        // Number is too small
        assert_eq!(is_valid_part2(10), false);
        assert_eq!(is_valid_part2(99999), false);
        // Number is too big
        assert_eq!(is_valid_part2(1111111), false);
        // No adjacent numbers that are the same
        assert_eq!(is_valid_part2(123456), false);
    }

    #[test]
    fn test_part1() {
        let input = vec![String::from("111111-111112")];
        assert_eq!(part1(&input), 2);

        let input = util::lines_from_file("./input/day04.txt");
        assert_eq!(part1(&input), 1855)
    }

    #[test]
    fn test_part2() {
        let input = vec![String::from("111111-111112")];
        assert_eq!(part2(&input), 0);
        let input = vec![String::from("111111-111122")];
        assert_eq!(part2(&input), 1);

        let input = util::lines_from_file("./input/day04.txt");
        assert_eq!(part2(&input), 1253)
    }

    #[test]
    fn test_consecutive_digits() {
        assert_eq!(has_max_consecutive_digits(123, 2), false);
        assert_eq!(has_max_consecutive_digits(112233, 2), true);
        assert_eq!(has_max_consecutive_digits(111222333, 2), false);
        assert_eq!(has_max_consecutive_digits(111222333, 3), true);
        assert_eq!(has_max_consecutive_digits(111222333, 4), true);
        assert_eq!(has_max_consecutive_digits(111222333, 5), true);
    }
}
