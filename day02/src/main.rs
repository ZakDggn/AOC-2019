use intcode::Program;

fn output(mut memory: Vec<i32>, noun: i32, verb: i32) -> i32 {
    memory[1] = noun;
    memory[2] = verb;
    let mut program = Program::new(memory);
    program.run("".as_bytes(), Vec::new());
    program.memory()[0]
}

fn main() {
    let memory = intcode::read_program_file("input").unwrap();

    let part1 = output(memory.clone(), 12, 2);
    println!("{part1}");

    for noun in 0..=99 {
        for verb in 0..=99 {
            if output(memory.clone(), noun, verb) == 19690720 {
                println!("{}", 100 * noun + verb);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_utils::assert_memory_eq;

    #[test]
    fn examples() {
        assert_memory_eq(
            &[1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
            &[3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
        );
        assert_memory_eq(&[1, 0, 0, 0, 99], &[2, 0, 0, 0, 99]);
        assert_memory_eq(&[2, 3, 0, 3, 99], &[2, 3, 0, 6, 99]);
        assert_memory_eq(&[2, 4, 4, 5, 99, 0], &[2, 4, 4, 5, 99, 9801]);
        assert_memory_eq(
            &[1, 1, 1, 4, 99, 5, 6, 0, 99],
            &[30, 1, 1, 4, 2, 5, 6, 0, 99],
        );
    }

    #[test]
    fn answers() {
        let memory = intcode::read_program_file("input").unwrap();
        assert_eq!(output(memory.clone(), 12, 2), 9581917);
        assert_eq!(output(memory.clone(), 25, 5), 19690720);
    }
}
