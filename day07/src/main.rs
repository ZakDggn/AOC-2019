use itertools::Itertools;

use intcode::Program;

fn run_amplifiers(program: &Program, phases: Vec<i32>) -> i32 {
    let mut programs = Vec::new();
    for phase in phases {
        let mut program = program.clone();
        program.run_with_input(phase, Vec::new());
        programs.push(program);
    }
    let mut prev_output = 0;
    let mut halted = false;
    while !halted {
        for program in &mut programs {
            let mut output: Vec<u8> = Vec::new();
            halted = program.run_with_input(prev_output, &mut output);
            prev_output = String::from_utf8(output).unwrap().trim().parse().unwrap();
        }
    }
    prev_output
}

fn highest_signal(program: &Program, phases: [i32; 5]) -> i32 {
    let mut outputs = Vec::new();
    for permutation in phases.into_iter().permutations(5) {
        outputs.push(run_amplifiers(program, permutation));
    }
    *outputs.iter().max().unwrap()
}

fn solve() -> (i32, i32) {
    let program = Program::from_file("input").unwrap();
    let part1 = highest_signal(&program, [0, 1, 2, 3, 4]);
    let part2 = highest_signal(&program, [5, 6, 7, 8, 9]);
    (part1, part2)
}

fn main() {
    let (part1, part2) = solve();
    println!("{part1}");
    println!("{part2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_with_input_simple() {
        let mut program = Program::from_file("../day07/input").unwrap();
        assert!(!program.run_with_input(0, Vec::new()));
        assert!(program.run_with_input(0, Vec::new()));
    }

    #[test]
    fn run_with_input_many() {
        let mut program = Program::from_file("../day07/input").unwrap();
        assert!(!program.run_with_input(5, Vec::new()));
        for _ in 0..9 {
            assert!(!program.run_with_input(0, Vec::new()));
        }
        assert!(program.run_with_input(0, Vec::new()));
    }

    #[test]
    fn examples() {
        todo!()
    }

    #[test]
    fn answers() {
        let (part1, part2) = solve();
        assert_eq!(part1, 21760);
        assert_eq!(part2, 69816958);
    }
}
