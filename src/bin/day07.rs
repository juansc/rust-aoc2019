use aoc2019::int_code;
use aoc2019::util;

fn main() {
    let memory: Vec<i64> = util::read_int_code_memory("./input/day07.txt");
    println!("{}", part1(&memory));
    println!("{}", part2(&memory));
}

fn part1(program: &[i64]) -> i64 {
    let num_apps = 5;
    util::Permutations::new(num_apps)
        .into_iter()
        .map(|perm| run_linear_configuration(program.to_owned(), perm))
        .max()
        .unwrap()
}

fn run_linear_configuration(program: Vec<i64>, phase_settings: Vec<u64>) -> i64 {
    let mut amps: Vec<int_code::IntCodeComputer> =
        Vec::with_capacity(phase_settings.len() as usize);
    for phase in phase_settings {
        let mut comp = int_code::IntCodeComputer::new(program.clone());
        comp.input.write(phase as i64);
        amps.push(comp);
    }

    let mut output = 0i64;
    for comp in amps.iter_mut() {
        comp.input.write(output);
        comp.run();
        output = *comp.dump_output().read_all().first().unwrap();
    }
    output
}

fn part2(program: &[i64]) -> i64 {
    let num_apps = 5;
    util::Permutations::new(num_apps)
        .into_iter()
        .map(|perm| run_circular_configuration(program.to_owned(), perm))
        .max()
        .unwrap()
}

fn run_circular_configuration(program: Vec<i64>, phase_settings: Vec<u64>) -> i64 {
    let mut amps: Vec<int_code::IntCodeComputer> =
        Vec::with_capacity(phase_settings.len() as usize);
    for phase in phase_settings {
        let mut comp = int_code::IntCodeComputer::new(program.clone());
        // The shift settings for the new amplifiers are 5-9
        comp.input.write(5 + phase as i64);
        amps.push(comp);
    }

    let mut output = 0i64;
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
        let memory: Vec<i64> = util::read_int_code_memory("./input/day07.txt");

        assert_eq!(part1(&memory), 199988);
    }

    #[test]
    fn test_part2() {
        let memory: Vec<i64> = util::read_int_code_memory("./input/day07.txt");

        assert_eq!(part2(&memory), 17519904);
    }
}
