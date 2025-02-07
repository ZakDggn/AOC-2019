use intcode::Program;

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
