use std::collections::HashMap;

use intcode::Program;

type Coord = (i32, i32);

enum Color {
    Black,
    White,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn turn_left(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Left,
        Direction::Down => Direction::Right,
        Direction::Left => Direction::Down,
        Direction::Right => Direction::Up,
    }
}

fn turn_right(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::Right => Direction::Down,
    }
}

fn move_forward((x, y): Coord, direction: &Direction) -> Coord {
    let (dx, dy) = match direction {
        Direction::Up => (0, -1),
        Direction::Down => (0, 1),
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0),
    };
    (x + dx, y + dy)
}

fn run_robot(mut program: Program, starting_color: Color) -> HashMap<Coord, Color> {
    let mut direction = Direction::Up;
    let mut coord = (0, 0);
    let mut panels = HashMap::from([(coord, starting_color)]);

    let mut halted = false;
    while !halted {
        let input = match panels.get(&coord) {
            Some(Color::White) => 1,
            _ => 0,
        };
        let mut output = Vec::new();
        halted = program.run_with_input(input, &mut output);
        let outputs: Vec<&str> = std::str::from_utf8(&output).unwrap().split('\n').collect();
        let color = match outputs[0] {
            "0" => Color::Black,
            "1" => Color::White,
            s => panic!("bad color output '{s}'"),
        };
        direction = match outputs[1] {
            "0" => turn_left(&direction),
            "1" => turn_right(&direction),
            s => panic!("bad direction output '{s}'"),
        };
        panels.insert(coord, color);
        coord = move_forward(coord, &direction);
    }

    panels
}

fn show(panels: &HashMap<Coord, Color>) {
    let mut max_x = 0;
    let mut min_x = 0;
    let mut max_y = 0;
    let mut min_y = 0;
    for &(x, y) in panels.keys() {
        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
    }
    for y in min_y..=max_y {
        let mut row = String::new();
        for x in min_x..=max_x {
            row.push(match panels.get(&(x, y)) {
                Some(Color::White) => '#',
                _ => ' ',
            });
        }
        println!("{row}");
    }
}

fn main() {
    let program = Program::from_file("input").unwrap();

    let part1 = run_robot(program.clone(), Color::Black).len();
    println!("{part1}");

    let panels = run_robot(program, Color::White);
    show(&panels);
}
