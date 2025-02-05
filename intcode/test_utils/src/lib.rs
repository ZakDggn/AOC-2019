use std::io::Read;

pub fn assert_memory_eq(memory: &[i32], expected: &[i32]) {
    let mut memory = memory.to_vec();
    intcode::run(&mut memory, "".as_bytes(), Vec::new());
    assert_eq!(memory, expected);
}

pub fn assert_output_eq(mut memory: Vec<i32>, input: &str, expected: &str) {
    let mut output = Vec::new();
    intcode::run(&mut memory, input.as_bytes(), &mut output);
    let mut buf = String::new();
    output.as_slice().read_to_string(&mut buf).unwrap();
    assert_eq!(buf, expected);
}
