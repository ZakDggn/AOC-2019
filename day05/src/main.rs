fn main() {
    let mut memory = intcode::read_file("input").unwrap();
    intcode::run(&mut memory, std::io::stdin().lock(), std::io::stdout());
}
