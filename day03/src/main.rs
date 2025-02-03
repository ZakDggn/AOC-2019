use std::collections::HashMap;
use std::fs;

type CoordMap = HashMap<(i32, i32), u32>;

fn path_coords(wire_path: &[&str]) -> CoordMap {
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

fn find_intersections(coords1: CoordMap, coords2: &CoordMap) -> CoordMap {
    let mut intersections = HashMap::new();
    for (coord, steps1) in coords1 {
        if let Some(steps2) = coords2.get(&coord) {
            intersections.insert(coord, steps1 + steps2);
        }
    }
    intersections
}

fn shortest_distance(intersections: &CoordMap) -> u32 {
    // closest intersection
    let (x, y) = intersections
        .keys()
        .min_by(|(x1, y1), (x2, y2)| (x1.abs() + y1.abs()).cmp(&(x2.abs() + y2.abs())))
        .expect("wires should intersect at least once");

    x.unsigned_abs() + y.unsigned_abs()
}

fn fewest_steps(intersections: &CoordMap) -> u32 {
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
    let coords1 = path_coords(&wires[0]);
    let coords2 = path_coords(&wires[1]);
    let intersections = find_intersections(coords1, &coords2);

    let part1 = shortest_distance(&intersections);
    println!("{part1}");

    let part2 = fewest_steps(&intersections);
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

    fn assert_shortest_distance(wire1: &str, wire2: &str, expected: u32) {
        let path1: Vec<_> = wire1.split(',').collect();
        let path2: Vec<_> = wire2.split(',').collect();
        let coords1 = path_coords(&path1);
        let coords2 = path_coords(&path2);
        let intersections = find_intersections(coords1, &coords2);
        assert_eq!(shortest_distance(&intersections), expected);
    }

    #[test]
    fn part1_examples() {
        assert_shortest_distance("R8,U5,L5,D3", "U7,R6,D4,L4", 6);
        assert_shortest_distance(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72",
            "U62,R66,U55,R34,D71,R55,D58,R83",
            159,
        );
        assert_shortest_distance(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
            135,
        );
    }

    fn assert_fewest_steps(wire1: &str, wire2: &str, expected: u32) {
        let path1: Vec<_> = wire1.split(',').collect();
        let path2: Vec<_> = wire2.split(',').collect();
        let coords1 = path_coords(&path1);
        let coords2 = path_coords(&path2);
        let intersections = find_intersections(coords1, &coords2);
        assert_eq!(fewest_steps(&intersections), expected);
    }

    #[test]
    fn part2_examples() {
        assert_fewest_steps("R8,U5,L5,D3", "U7,R6,D4,L4", 30);
        assert_fewest_steps(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72",
            "U62,R66,U55,R34,D71,R55,D58,R83",
            610,
        );
        assert_fewest_steps(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
            410,
        );
    }
}
