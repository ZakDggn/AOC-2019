use std::{collections::HashMap, fs};

type Map<'a> = HashMap<&'a str, &'a str>;

/// parse `map_data` into a map from each child object to its parent
fn parse(map_data: &str) -> Map {
    map_data
        .lines()
        .map(|line| {
            let (parent, child) = line.split_once(')').unwrap();
            (child, parent)
        })
        .collect()
}

/// find total number of direct and indirect orbits in `map_data`
fn orbits(map: &Map) -> u32 {
    let mut counts = HashMap::from([("COM", 0)]);
    for object in map.keys() {
        count(&mut counts, map, object);
    }

    counts.values().sum()
}

/// set `object`'s parent orbit count recursively, then set `object`'s orbit count
fn count<'a>(counts: &mut HashMap<&'a str, u32>, map: &'a Map, object: &'a str) {
    if counts.contains_key(object) {
        return;
    }
    let parent = map.get(object).unwrap();
    count(counts, map, parent);
    let parent_count = counts.get(parent).unwrap();
    counts.insert(object, parent_count + 1);
}

/// returns path from COM to (but not including) `object`
fn path_from_com<'a>(map: &'a Map, object: &'a str) -> Vec<&'a str> {
    let mut path = Vec::new();
    let mut current = object;
    while current != "COM" {
        current = map.get(current).unwrap();
        path.push(current);
    }
    path.reverse();
    path
}

/// returns the minimum number of orbital transfers required to move from
/// the object `a` is orbiting to the object `b` is orbiting
fn transfers(map: &Map, a: &str, b: &str) -> usize {
    let path_to_a = path_from_com(map, a);
    let path_to_b = path_from_com(map, b);
    let mut i = 0;
    while path_to_a.get(i) == path_to_b.get(i) {
        i += 1;
    }
    (path_to_a.len() - i) + (path_to_b.len() - i)
}

fn main() {
    let map_data = fs::read_to_string("input").unwrap();
    let map = parse(&map_data);

    let part1 = orbits(&map);
    println!("{part1}");
    let part2 = transfers(&map, "YOU", "SAN");
    println!("{part2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let map_data = fs::read_to_string("example").unwrap();
        let map = parse(&map_data);
        assert_eq!(orbits(&map), 42);
    }

    #[test]
    fn test_path_from_com() {
        let map_data = fs::read_to_string("example").unwrap() + "K)YOU\nI)SAN\n";
        let map = parse(&map_data);

        assert_eq!(path_from_com(&map, "H"), ["COM", "B", "G"]);
        assert_eq!(
            path_from_com(&map, "YOU"),
            ["COM", "B", "C", "D", "E", "J", "K"]
        );
        assert_eq!(path_from_com(&map, "SAN"), ["COM", "B", "C", "D", "I"]);
    }

    #[test]
    fn test_transfers() {
        let map_data = fs::read_to_string("example").unwrap();
        let map = parse(&map_data);
        assert_eq!(transfers(&map, "F", "C"), 3);
        assert_eq!(transfers(&map, "L", "B"), 6);
        assert_eq!(transfers(&map, "L", "H"), 6);
    }

    #[test]
    fn part2_example() {
        let map_data = fs::read_to_string("example").unwrap() + "K)YOU\nI)SAN\n";
        let map = parse(&map_data);
        assert_eq!(transfers(&map, "YOU", "SAN"), 4);
    }
}
