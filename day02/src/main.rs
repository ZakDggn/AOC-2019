use std::fs;

fn run(program: &mut [usize]) {
    let mut ip = 0;
    while program[ip] != 99 {
        let opcode = program[ip];
        let param1 = program[program[ip + 1]];
        let param2 = program[program[ip + 2]];
        let addr = program[ip + 3];
        match opcode {
            1 => program[addr] = param1 + param2,
            2 => program[addr] = param1 * param2,
            _ => panic!("Unknown opcode: {opcode}"),
        }
        ip += 4;
    }
}

fn output(program: &[usize], noun: usize, verb: usize) -> usize {
    let mut program = program.to_vec();
    program[1] = noun;
    program[2] = verb;
    run(&mut program);
    program[0]
}

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let program: Vec<usize> = contents
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let part1 = output(&program, 12, 2);
    println!("{part1}");

    for noun in 0..=99 {
        for verb in 0..=99 {
            if output(&program, noun, verb) == 19690720 {
                println!("{}", 100 * noun + verb);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn example_programs() {
        let mut program = [1, 0, 0, 0, 99];
        run(&mut program);
        assert_eq!(program, [2, 0, 0, 0, 99]);

        let mut program = [2, 3, 0, 3, 99];
        run(&mut program);
        assert_eq!(program, [2, 3, 0, 6, 99]);

        let mut program = [2, 4, 4, 5, 99, 0];
        run(&mut program);
        assert_eq!(program, [2, 4, 4, 5, 99, 9801]);

        let mut program = [1, 1, 1, 4, 99, 5, 6, 0, 99];
        run(&mut program);
        assert_eq!(program, [30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
