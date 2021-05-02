use aoc2019::util;

fn part1(memory: &Vec<i32>) -> i32 {
    let mut mem = memory.clone();
    // Set the noun and verb according to puzzle
    mem[1] = 12;
    mem[2] = 2;
    let mut computer1 = util::IntCodeComputer::new(mem);
    computer1.run();
    return computer1.dump_memory().read(0);
}

fn part2(memory: &Vec<i32>) -> i32 {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut mem = memory.clone();
            mem[1] = noun;
            mem[2] = verb;
            let mut computer = util::IntCodeComputer::new(mem.clone());
            computer.run();
            if computer.dump_memory().read(0) == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    panic!(
        "Could not find noun and verb such that the output was {}",
        19690720
    )
}

fn main() {
    let memory: Vec<i32> = util::lines_from_file("./input/day02.txt")
        .first()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<i32>())
        .filter_map(Result::ok)
        .collect();

    println!("Solution for part 1: {}", part1(&memory));
    println!("Solution for part 2: {}", part2(&memory));
}

// Tests
#[cfg(test)]
mod tests {
    use aoc2019::util;

    use crate::{part1, part2};

    #[test]
    fn test_part1() {
        let memory: Vec<i32> = util::lines_from_file("./input/day02.txt")
            .first()
            .unwrap()
            .split(",")
            .map(|x| x.parse::<i32>())
            .filter_map(Result::ok)
            .collect();

        assert_eq!(part1(&memory), 4484226);
    }

    #[test]
    fn test_part2() {
        let memory: Vec<i32> = util::lines_from_file("./input/day02.txt")
            .first()
            .unwrap()
            .split(",")
            .map(|x| x.parse::<i32>())
            .filter_map(Result::ok)
            .collect();

        assert_eq!(part2(&memory), 5696);
    }
}
