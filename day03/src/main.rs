use std::collections::HashMap;
use std::fs;

fn path_coords(wire_path: &[&str]) -> HashMap<(i32, i32), u32> {
    let (mut x, mut y) = (0, 0);
    let mut coords = HashMap::new();
    let mut steps = 0;
    for line in wire_path {
        let mut chars = line.chars();
        let (dx, dy) = match chars.next().unwrap() {
            'R' => (1, 0),
            'L' => (-1, 0),
            'U' => (0, 1),
            'D' => (0, -1),
            _ => unreachable!(),
        };
        let length: i32 = chars.as_str().parse().unwrap();
        for _ in 0..length {
            (x, y) = (x + dx, y + dy);
            steps += 1;
            coords.entry((x, y)).or_insert(steps);
        }
    }
    coords
}

fn shortest_distance(wire_path1: &[&str], wire_path2: &[&str]) -> u32 {
    let coords1 = path_coords(wire_path1);
    let coords2 = path_coords(wire_path2);
    let mut intersections = Vec::new();
    for &coord in coords1.keys() {
        if coords2.contains_key(&coord) {
            intersections.push(coord);
        }
    }
    // closest intersection
    let (x, y) = intersections
        .iter()
        .min_by(|(x1, y1), (x2, y2)| (x1.abs() + y1.abs()).cmp(&(x2.abs() + y2.abs())))
        .expect("wires should intersect at least once");

    x.unsigned_abs() + y.unsigned_abs()
}

fn fewest_steps(wire_path1: &[&str], wire_path2: &[&str]) -> u32 {
    let coords1 = path_coords(wire_path1);
    let coords2 = path_coords(wire_path2);
    let mut intersections = HashMap::new();
    for (coord, steps1) in coords1 {
        if let Some(steps2) = coords2.get(&coord) {
            intersections.insert(coord, steps1 + steps2);
        }
    }
    *intersections
        .values()
        .min()
        .expect("wires should intersect at least once")
}

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let wires: Vec<Vec<&str>> = contents
        .lines()
        .map(|line| line.trim().split(',').collect())
        .collect();

    let part1 = shortest_distance(&wires[0], &wires[1]);
    println!("{part1}");

    let part2 = fewest_steps(&wires[0], &wires[1]);
    println!("{part2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_coords() {
        let wire_path: Vec<&str> = "R3,U2,L5,D4".split(',').collect();
        let actual = path_coords(&wire_path);
        let expected = HashMap::from([
            ((1, 0), 1),
            ((2, 0), 2),
            ((3, 0), 3),
            ((3, 1), 4),
            ((3, 2), 5),
            ((2, 2), 6),
            ((1, 2), 7),
            ((0, 2), 8),
            ((-1, 2), 9),
            ((-2, 2), 10),
            ((-2, 1), 11),
            ((-2, 0), 12),
            ((-2, -1), 13),
            ((-2, -2), 14),
        ]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn self_overlap() {
        let wire_path: Vec<&str> = "R3,U2,L1,D4".split(',').collect();
        let actual = path_coords(&wire_path);
        let expected = HashMap::from([
            ((1, 0), 1),
            ((2, 0), 2),
            ((3, 0), 3),
            ((3, 1), 4),
            ((3, 2), 5),
            ((2, 2), 6),
            ((2, 1), 7),
            // (2, 0) already visited
            ((2, -1), 9),
            ((2, -2), 10),
        ]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn part1_examples() {
        let path1: Vec<_> = "R8,U5,L5,D3".split(',').collect();
        let path2: Vec<_> = "U7,R6,D4,L4".split(',').collect();
        assert_eq!(shortest_distance(&path1, &path2), 6);

        let path1: Vec<_> = "R75,D30,R83,U83,L12,D49,R71,U7,L72".split(',').collect();
        let path2: Vec<_> = "U62,R66,U55,R34,D71,R55,D58,R83".split(',').collect();
        assert_eq!(shortest_distance(&path1, &path2), 159);

        let path1: Vec<_> = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"
            .split(',')
            .collect();
        let path2: Vec<_> = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".split(',').collect();
        assert_eq!(shortest_distance(&path1, &path2), 135);
    }

    #[test]
    fn part2_examples() {
        let path1: Vec<_> = "R8,U5,L5,D3".split(',').collect();
        let path2: Vec<_> = "U7,R6,D4,L4".split(',').collect();
        assert_eq!(fewest_steps(&path1, &path2), 30);

        let path1: Vec<_> = "R75,D30,R83,U83,L12,D49,R71,U7,L72".split(',').collect();
        let path2: Vec<_> = "U62,R66,U55,R34,D71,R55,D58,R83".split(',').collect();
        assert_eq!(fewest_steps(&path1, &path2), 610);

        let path1: Vec<_> = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"
            .split(',')
            .collect();
        let path2: Vec<_> = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".split(',').collect();
        assert_eq!(fewest_steps(&path1, &path2), 410);
    }
}
