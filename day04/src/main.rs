use std::fs;

fn get_digits(mut n: u32) -> Vec<u8> {
    let mut digits = Vec::new();
    while n != 0 {
        digits.push((n % 10) as u8);
        n /= 10;
    }
    digits.reverse();
    digits
}

fn is_valid_part1(digits: &[u8]) -> bool {
    for (a, b) in digits.iter().zip(&digits[1..]) {
        if a == b {
            return true;
        }
    }
    false
}

fn is_valid_part2(digits: &[u8]) -> bool {
    digits
        .iter()
        .any(|x| digits.iter().filter(|&y| y == x).count() == 2)
}

fn find_passwords(min: u32, max: u32) -> (Vec<u32>, Vec<u32>) {
    assert!(min >= 100_000 && max < 1_000_000);
    let mut passwords1 = Vec::new();
    let mut passwords2 = Vec::new();
    for n in min..=max {
        let digits = get_digits(n);
        if !digits.is_sorted() {
            continue;
        }
        if is_valid_part2(&digits) {
            passwords1.push(n);
            passwords2.push(n);
        } else if is_valid_part1(&digits) {
            passwords1.push(n);
        }
    }
    (passwords1, passwords2)
}

fn main() {
    let nums: Vec<u32> = fs::read_to_string("input")
        .unwrap()
        .trim()
        .split('-')
        .map(|n| n.parse().unwrap())
        .collect();
    let min = nums[0];
    let max = nums[1];

    let (passwords1, passwords2) = find_passwords(min, max);
    println!("{}", passwords1.len());
    println!("{}", passwords2.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_digits() {
        assert_eq!(get_digits(1), [1]);
        assert_eq!(get_digits(123), [1, 2, 3]);
        assert_eq!(get_digits(987654321), [9, 8, 7, 6, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_is_valid_part1() {
        assert!(is_valid_part1(&get_digits(111111)));
        assert!(!is_valid_part1(&get_digits(123789)));
    }

    #[test]
    fn test_is_valid_part2() {
        assert!(!is_valid_part2(&get_digits(111111)));
        assert!(!is_valid_part2(&get_digits(123789)));
        assert!(is_valid_part2(&get_digits(112233)));
        assert!(!is_valid_part2(&get_digits(123444)));
        assert!(is_valid_part2(&get_digits(111122)));
    }
}
