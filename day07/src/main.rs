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

fn main() {
    let program = Program::from_file("input").unwrap();

    let part1 = highest_signal(&program, [0, 1, 2, 3, 4]);
    println!("{part1}");

    let part2 = highest_signal(&program, [5, 6, 7, 8, 9]);
    println!("{part2}");
}
