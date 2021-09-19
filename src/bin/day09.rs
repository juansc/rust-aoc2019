use aoc2019::{int_code, util};

const INPUT_FILE: &str = "./input/day09.txt";

fn part1(memory: &[i64]) -> Vec<i64> {
    let mut mem = memory.to_owned();
    // Set the noun and verb according to puzzle
    let mut computer1 = int_code::IntCodeComputer::new(mem);
    let input_stream = int_code::DataStream::new();
    computer1.attach_input(input_stream);
    computer1.input.write(1);
    computer1.run();
    computer1.dump_output().read_all().to_vec()
}

fn part2(memory: &[i64]) -> i64 {
    0i64
}

fn main() {
    let memory: Vec<i64> = util::read_int_code_memory(INPUT_FILE);

    println!("Solution for part 1: {:?}", part1(&memory));
    println!("Solution for part 2: {}", part2(&memory));
}

// Tests
#[cfg(test)]
mod tests {
    use aoc2019::util;

    use crate::{part1, part2, INPUT_FILE};

    #[test]
    fn test_part1() {
        let memory: Vec<i64> = util::read_int_code_memory(INPUT_FILE);

        assert_eq!(part1(&memory), 4484226);
    }

    #[test]
    fn test_part2() {
        let memory: Vec<i64> = util::read_int_code_memory(INPUT_FILE);

        assert_eq!(part2(&memory), 5696);
    }
}
