use aoc2019::{int_code, util};

fn main() {
    let memory: Vec<i32> = util::read_int_code_memory("./input/day05.txt");

    println!("Part 1 Solution: {}", part1(&memory));
}

fn part1(memory: &[i32]) -> i32 {
    let mem = memory.to_owned();
    // Set the noun and verb according to puzzle
    let mut computer1 = int_code::IntCodeComputer::new(mem);
    let input_stream = int_code::DataStream::new();
    computer1.attach_input(input_stream);
    computer1.input.write(1);
    computer1.run();
    let output_stream = computer1.dump_output().read_all();
    for (ind, val) in output_stream.iter().enumerate() {
        if *val != 0 && ind != output_stream.len() - 1 {
            panic!("test failed, expected a non-zero output for test diagnostic")
        }
    }
    *output_stream.last().unwrap()
}

fn part2(memory: &[i32]) -> i32 {
    let mem = memory.to_owned();
    // Set the noun and verb according to puzzle
    let mut computer1 = int_code::IntCodeComputer::new(mem);
    computer1.input.write(5);
    computer1.run();
    let output_stream = computer1.dump_output().read_all();
    *output_stream.last().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    use aoc2019::util;

    #[test]
    fn test_part1() {
        let memory: Vec<i32> = util::read_int_code_memory("./input/day05.txt");

        assert_eq!(part1(&memory), 15386262);
    }

    #[test]
    fn test_part2() {
        let memory: Vec<i32> = util::read_int_code_memory("./input/day05.txt");

        assert_eq!(part2(&memory), 10376124);
    }
}
