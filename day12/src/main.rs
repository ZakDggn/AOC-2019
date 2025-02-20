use std::{cmp::Ordering, fs};

#[derive(Debug, Copy, Clone, PartialEq)]
struct V3 {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Copy, Clone)]
struct Moon {
    pos: V3,
    vel: V3,
}

impl Moon {
    fn apply_gravity(&mut self, other: &Self) {
        fn apply_axis(vel: &mut i32, a: i32, b: i32) {
            *vel += match a.cmp(&b) {
                Ordering::Less => 1,
                Ordering::Equal => 0,
                Ordering::Greater => -1,
            }
        }
        apply_axis(&mut self.vel.x, self.pos.x, other.pos.x);
        apply_axis(&mut self.vel.y, self.pos.y, other.pos.y);
        apply_axis(&mut self.vel.z, self.pos.z, other.pos.z);
    }

    fn apply_velocity(&mut self) {
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;
        self.pos.z += self.vel.z;
    }

    fn energy(&self) -> i32 {
        let potential = self.pos.x.abs() + self.pos.y.abs() + self.pos.z.abs();
        let kinetic = self.vel.x.abs() + self.vel.y.abs() + self.vel.z.abs();
        potential * kinetic
    }
}

fn parse_line(line: &str) -> Vec<i32> {
    line.split(',')
        .map(|s| {
            s.chars()
                .filter(|&c| c.is_ascii_digit() || c == '-')
                .collect::<String>()
                .parse()
                .unwrap()
        })
        .collect()
}

fn parse(contents: &str) -> Vec<Moon> {
    contents
        .lines()
        .map(|line| {
            let nums = parse_line(line);
            Moon {
                pos: V3 {
                    x: nums[0],
                    y: nums[1],
                    z: nums[2],
                },
                vel: V3 { x: 0, y: 0, z: 0 },
            }
        })
        .collect()
}

fn step(moons: &mut Vec<Moon>) {
    for i in 0..moons.len() {
        let mut moon = moons[i];
        for (j, other) in moons.iter().enumerate() {
            if i != j {
                moon.apply_gravity(other);
            }
        }
        moons[i] = moon;
    }
    for moon in moons {
        moon.apply_velocity();
    }
}

fn simulate(mut moons: Vec<Moon>, n: u32) -> i32 {
    for _ in 0..n {
        step(&mut moons);
    }
    moons.iter().map(Moon::energy).sum()
}

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let moons = parse(&contents);

    let part1 = simulate(moons, 1000);
    println!("{part1}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moon() {
        let mut moon = Moon {
            pos: V3 { x: 5, y: -3, z: 6 },
            vel: V3 { x: -2, y: 1, z: 3 },
        };
        let other = Moon {
            pos: V3 { x: 0, y: -3, z: 7 },
            vel: V3 { x: 0, y: 0, z: 0 },
        };
        moon.apply_gravity(&other);
        moon.apply_velocity();
        assert_eq!(moon.pos, V3 { x: 2, y: -2, z: 10 });
        assert_eq!(moon.energy(), 112);
    }

    #[test]
    fn part1_examples() {
        let contents = fs::read_to_string("example1").unwrap();
        let moons = parse(&contents);
        assert_eq!(simulate(moons, 10), 179);

        let contents = fs::read_to_string("example2").unwrap();
        let moons = parse(&contents);
        assert_eq!(simulate(moons, 100), 1940);
    }
}
