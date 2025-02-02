use std::fs;

fn path_coords(wire_path: &[&str]) -> Vec<(i32, i32)> {
    let (mut x, mut y) = (0, 0);
    let mut coords = vec![];
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
            coords.push((x, y));
        }
    }
    coords
}

fn closest_intersection(coords1: &[(i32, i32)], coords2: &[(i32, i32)]) -> Option<(i32, i32)> {
    coords1
        .iter()
        .filter(|&x| coords2.contains(x))
        .min_by(|(x1, y1), (x2, y2)| (x1.abs() + y1.abs()).cmp(&(x2.abs() + y2.abs())))
        .copied()
}

fn shortest_distance(wire_path1: &[&str], wire_path2: &[&str]) -> u32 {
    let coords1 = path_coords(wire_path1);
    let coords2 = path_coords(wire_path2);
    let (x, y) = closest_intersection(&coords1, &coords2).unwrap();
    x.unsigned_abs() + y.unsigned_abs()
}

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let wires: Vec<Vec<&str>> = contents
        .lines()
        .map(|line| line.trim().split(',').collect())
        .collect();

    let distance = shortest_distance(&wires[0], &wires[1]);
    println!("{distance}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_coords() {
        let wire_path: Vec<&str> = "R3,U2,L5,D4".split(',').collect();
        let actual = path_coords(&wire_path);
        let expected = [
            (1, 0),
            (2, 0),
            (3, 0),
            (3, 1),
            (3, 2),
            (2, 2),
            (1, 2),
            (0, 2),
            (-1, 2),
            (-2, 2),
            (-2, 1),
            (-2, 0),
            (-2, -1),
            (-2, -2),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_closest_intersection() {
        let path1: Vec<_> = "R8,U5,L5,D3".split(',').collect();
        let path2: Vec<_> = "U7,R6,D4,L4".split(',').collect();
        let coords1 = path_coords(&path1);
        let coords2 = path_coords(&path2);
        assert_eq!(closest_intersection(&coords1, &coords2), Some((3, 3)));
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
}
