use intcode::Program;

fn main() {
    let mut program = Program::from_file("input").unwrap();
    program.run(std::io::stdin().lock(), std::io::stdout());
}

#[cfg(test)]
mod tests {
    use test_utils::{assert_memory_eq, assert_output_eq};

    #[test]
    fn examples() {
        let memory = [3, 0, 4, 0, 99];
        let input = "-123\n";
        assert_output_eq(&memory, input, input);

        assert_memory_eq(&[1002, 4, 3, 4, 33], &[1002, 4, 3, 4, 99]);

        assert_memory_eq(&[1101, 100, -1, 4, 0], &[1101, 100, -1, 4, 99]);

        let memory = [3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_output_eq(&memory, "8", "1\n");
        assert_output_eq(&memory, "-123", "0\n");

        let memory = [3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_output_eq(&memory, "8", "0\n");
        assert_output_eq(&memory, "7", "1\n");

        let memory = [3, 3, 1108, -1, 8, 3, 4, 3, 99];
        assert_output_eq(&memory, "8", "1\n");
        assert_output_eq(&memory, "-123", "0\n");

        let memory = [3, 3, 1107, -1, 8, 3, 4, 3, 99];
        assert_output_eq(&memory, "8", "0\n");
        assert_output_eq(&memory, "7", "1\n");

        let memory = [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        assert_output_eq(&memory, "0", "0\n");
        assert_output_eq(&memory, "42", "1\n");

        let memory = [3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        assert_output_eq(&memory, "0", "0\n");
        assert_output_eq(&memory, "42", "1\n");

        let memory = [
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        assert_output_eq(&memory, "7", "999\n");
        assert_output_eq(&memory, "8", "1000\n");
        assert_output_eq(&memory, "9", "1001\n");
    }

    #[test]
    fn answers() {
        let memory = intcode::read_program_file("input").unwrap();
        assert_output_eq(&memory, "1", "0\n0\n0\n0\n0\n0\n0\n0\n0\n9961446\n");
        assert_output_eq(&memory, "5", "742621\n");
    }
}
