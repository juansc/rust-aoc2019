use aoc2019::{int_code, util};

fn main() {
    let memory: Vec<i32> = util::lines_from_file("./input/day05.txt")
        .first()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<i32>())
        .filter_map(Result::ok)
        .collect();

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
        if ind == output_stream.len() - 1 {
            return *val;
        }
    }
    0
}
