use std::io;

use intcode::Program;

fn main() {
    let mut program = Program::from_file("input").unwrap();
    program.run(io::stdin().lock(), io::stdout());
}

#[cfg(test)]
mod tests {
    use test_utils::assert_output_eq;

    #[test]
    fn examples() {
        let memory = [
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let expected = memory.map(|x| x.to_string()).join("\n") + "\n";
        assert_output_eq(&memory, "", &expected);

        assert_output_eq(
            &[1102, 34915192, 34915192, 7, 4, 7, 99, 0],
            "",
            "1219070632396864\n",
        );

        assert_output_eq(&[104, 1125899906842624, 99], "", "1125899906842624\n");
    }

    #[test]
    fn answers() {
        let memory = intcode::read_program_file("input").unwrap();
        assert_output_eq(&memory, "1", "3601950151\n");
        assert_output_eq(&memory, "2", "64236\n");
    }
}
