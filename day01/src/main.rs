use std::fs;

fn fuel(x: i32) -> i32 {
    let result = x / 3 - 2;
    if result > 0 {
        result + fuel(result)
    } else {
        0
    }
}

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let masses: Vec<i32> = contents.lines().map(|line| line.parse().unwrap()).collect();

    let part1: i32 = masses.iter().map(|&x| x / 3 - 2).sum();
    println!("{part1}");

    let part2: i32 = masses.iter().map(|&x| fuel(x)).sum();
    println!("{part2}");
}

#[cfg(test)]
mod tests {
    use super::fuel;

    #[test]
    fn fuel_calculation() {
        assert_eq!(fuel(14), 2);
        assert_eq!(fuel(1969), 966);
        assert_eq!(fuel(100756), 50346);
    }
}
