use intcode::Program;

pub fn assert_memory_eq(memory: &[i32], expected: &[i32]) {
    let mut program = Program::new(memory.to_vec());
    program.run("".as_bytes(), Vec::new());
    assert_eq!(program.memory(), expected);
}

pub fn assert_output_eq(memory: &[i32], input: &str, expected: &str) {
    let mut output = Vec::new();
    let mut program = Program::new(memory.to_vec());
    program.run(input.as_bytes(), &mut output);
    let output = String::from_utf8(output).unwrap();
    assert_eq!(output, expected);
}
