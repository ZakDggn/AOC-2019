use itertools::Itertools;

use intcode::Program;

fn run_amplifiers(program: &Program, phases: Vec<i64>) -> i64 {
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

fn highest_signal(program: &Program, phases: [i64; 5]) -> i64 {
    let mut outputs = Vec::new();
    for permutation in phases.into_iter().permutations(5) {
        outputs.push(run_amplifiers(program, permutation));
    }
    *outputs.iter().max().unwrap()
}

fn solve() -> (i64, i64) {
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
        let program = Program::new(vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ]);
        assert_eq!(highest_signal(&program, [4, 3, 2, 1, 0]), 43210);

        let program = Program::new(vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ]);
        assert_eq!(highest_signal(&program, [4, 3, 2, 1, 0]), 54321);

        let program = Program::new(vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ]);
        assert_eq!(highest_signal(&program, [1, 0, 4, 3, 2]), 65210);

        let program = Program::new(vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ]);
        assert_eq!(highest_signal(&program, [9, 8, 7, 6, 5]), 139629729);

        let program = Program::new(vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ]);
        assert_eq!(highest_signal(&program, [9, 7, 8, 5, 6]), 18216);
    }

    #[test]
    fn answers() {
        let (part1, part2) = solve();
        assert_eq!(part1, 21760);
        assert_eq!(part2, 69816958);
    }
}
