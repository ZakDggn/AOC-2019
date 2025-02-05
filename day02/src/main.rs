fn output(mut memory: Vec<i32>, noun: i32, verb: i32) -> i32 {
    memory[1] = noun;
    memory[2] = verb;
    intcode::run(&mut memory, "".as_bytes(), Vec::new());
    memory[0]
}

fn main() {
    let memory = intcode::read_file("input").unwrap();

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
