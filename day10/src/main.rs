use std::collections::HashMap;
use std::fs;

type Coord = (usize, usize);
type AsteroidMap = Vec<Vec<char>>;

fn parse(contents: &str) -> AsteroidMap {
    contents
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a.abs();
    }
    gcd(b, a % b)
}

fn is_visible(map: &AsteroidMap, (x1, y1): Coord, (x2, y2): Coord) -> bool {
    let (x1, y1) = (x1 as i32, y1 as i32);
    let (x2, y2) = (x2 as i32, y2 as i32);
    let (dx, dy) = (x2 - x1, y2 - y1);
    let gcd = gcd(dx, dy);
    let (dx, dy) = (dx / gcd, dy / gcd);
    let (mut x, mut y) = (x1 + dx, y1 + dy);
    while (x, y) != (x2, y2) {
        if map[y as usize][x as usize] == '#' {
            return false;
        }
        x += dx;
        y += dy;
    }
    true
}

fn count_visible(map: &AsteroidMap, asteroid: Coord) -> u32 {
    let mut count = 0;
    let height = map.len();
    let width = map[0].len();
    for y in 0..height {
        for x in 0..width {
            if (x, y) != asteroid && map[y][x] == '#' && is_visible(map, asteroid, (x, y)) {
                count += 1;
            }
        }
    }
    count
}

fn part1(map: &AsteroidMap) -> (Coord, u32) {
    let mut counts: HashMap<Coord, u32> = HashMap::new();
    let height = map.len();
    let width = map[0].len();
    for y in 0..height {
        for x in 0..width {
            if map[y][x] != '#' {
                continue;
            }
            let count = count_visible(map, (x, y));
            counts.insert((x, y), count);
        }
    }
    counts
        .into_iter()
        .max_by_key(|(_coord, count)| *count)
        .unwrap()
}

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let map = parse(&contents);
    println!("{:?}", part1(&map).1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(5, 10), 5);
        assert_eq!(gcd(8, 10), 2);
        assert_eq!(gcd(9, 10), 1);
        assert_eq!(gcd(12, -16), 4);
        assert_eq!(gcd(-12, 16), 4);
        assert_eq!(gcd(-12, -16), 4);
    }

    fn assert_answer_eq(file_path: &str, answer: (Coord, u32)) {
        let contents = fs::read_to_string(file_path).unwrap();
        let map = parse(&contents);
        assert_eq!(part1(&map), answer);
    }

    #[test]
    fn examples() {
        assert_answer_eq("example1", ((3, 4), 8));
        assert_answer_eq("example2", ((5, 8), 33));
        assert_answer_eq("example3", ((1, 2), 35));
        assert_answer_eq("example4", ((6, 3), 41));
        assert_answer_eq("example5", ((11, 13), 210));
    }
}
