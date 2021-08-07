// Ideas for how to run these computers
// 1. Create a "Kernel" struct or some kind of runner that owns all the computers and their i/o
//    and manages all of that.
// 2. Create a simple loop that does the same thing that Kernel would do. It may simplify the issues
//    with ownership.
// Those ideas are fine. But what about the actual loop? I think the approach should be as follows:
// 1. For a computer, have it run as far as it can until it stops. When it stops, ask it if it is waiting
// for input or whether it is finished. If it's waiting for input, skip it. If it is halted skip it.
// 2.
use aoc2019::int_code;
use aoc2019::util;
use std::cmp::max;

fn main() {
    let memory: Vec<i32> = util::read_int_code_memory("./input/day07.txt");
    println!("{}", part1(&memory));
    println!("{}", part2(&memory));
}

fn part1(program: &[i32]) -> i32 {
    let num_apps = 5;
    util::Permutations::new(num_apps)
        .into_iter()
        .map(|perm| run_linear_configuration(program.to_owned(), perm))
        .max()
        .unwrap()
}

fn run_linear_configuration(program: Vec<i32>, phase_settings: Vec<u32>) -> i32 {
    let mut amps: Vec<int_code::IntCodeComputer> =
        Vec::with_capacity(phase_settings.len() as usize);
    for phase in phase_settings {
        let mut comp = int_code::IntCodeComputer::new(program.clone());
        comp.input.write(phase as i32);
        amps.push(comp);
    }

    let mut output = 0i32;
    for comp in amps.iter_mut() {
        comp.input.write(output);
        comp.run();
        output = *comp.dump_output().read_all().first().unwrap();
    }
    output
}

fn part2(program: &[i32]) -> i32 {
    let num_apps = 5;
    util::Permutations::new(num_apps)
        .into_iter()
        .map(|perm| run_circular_configuration(program.to_owned(), perm))
        .max()
        .unwrap()
}

fn run_circular_configuration(program: Vec<i32>, phase_settings: Vec<u32>) -> i32 {
    let mut amps: Vec<int_code::IntCodeComputer> =
        Vec::with_capacity(phase_settings.len() as usize);
    for phase in phase_settings {
        let mut comp = int_code::IntCodeComputer::new(program.clone());
        // The shift settings for the new amplifiers are 5-9
        comp.input.write(5 + phase as i32);
        amps.push(comp);
    }

    let mut output = 0i32;
    while !amps.iter().all(|x| x.is_halted()) {
        for comp in amps.iter_mut() {
            comp.input.write(output);
            comp.run();
            output = *comp.dump_output().read_all().first().unwrap();
            comp.clear_output();
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    use aoc2019::util;

    #[test]
    fn test_part1() {
        let memory: Vec<i32> = util::read_int_code_memory("./input/day07.txt");

        assert_eq!(part1(&memory), 199988);
    }

    #[test]
    fn test_part2() {
        let memory: Vec<i32> = util::read_int_code_memory("./input/day07.txt");

        assert_eq!(part2(&memory), 17519904);
    }
}
