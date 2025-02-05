use std::{collections::HashMap, fs};

/// find total number of direct and indirect orbits in `map_data`
fn orbits(map_data: String) -> u32 {
    // map from each child object to its parent
    let map: HashMap<&str, &str> = map_data
        .lines()
        .map(|line| {
            let (b, a) = line.split_once(')').unwrap();
            (a, b)
        })
        .collect();
    // map from each object to number of direct + indirect orbits
    let mut counts = HashMap::from([("COM", 0)]);
    for object in map.keys() {
        count(&mut counts, &map, object);
    }

    counts.values().sum()
}

/// set `object`'s parent orbit count recursively, then set `object`'s orbit count
fn count<'a>(counts: &mut HashMap<&'a str, u32>, map: &HashMap<&'a str, &'a str>, object: &'a str) {
    if counts.contains_key(object) {
        return;
    }
    let parent = map.get(object).unwrap();
    count(counts, map, parent);
    let parent_count = counts.get(parent).unwrap();
    counts.insert(object, parent_count + 1);
}

fn main() {
    let map_data = fs::read_to_string("input").unwrap();
    println!("{}", orbits(map_data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let map_data = fs::read_to_string("example").unwrap();
        assert_eq!(orbits(map_data), 42);
    }
}
