use array_tool::vec::Intersect;
use std::cmp::PartialEq;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    let mut line_rdr = BufReader::new(File::open("src/input.txt").unwrap()).lines();
    let mut wires = Vec::new();
    while let Some(Ok(line)) = line_rdr.next() {
        let wire: Vec<Direction> = line.split(',').map(|s| s.parse().unwrap()).collect();
        wires.push(wire);
    }

    let mut wire_maps = Vec::new();
    for wire in &wires {
        let mut wire_points = Vec::new();
        let mut pos = (0, 0);
        for dir in wire {
            for _ in 0..dir.len() {
                pos = dir.mv(pos);
                wire_points.push(pos);
            }
        }
        wire_maps.push(wire_points);
    }

    let intersecting_points: Vec<Position> =
        wire_maps[0].intersect_if(wire_maps[1].clone(), |l, r| l.0 == r.0 && l.1 == r.1);

    let mut distances: Vec<i32> = intersecting_points
        .iter()
        .map(|pos| pos.0.abs() + pos.1.abs())
        .collect();
    distances.sort();

    println!("Part 1: {}", distances[0]);
}

type Position = (i32, i32); // (x, y)

#[derive(Debug)]
enum Direction {
    Left(i32),
    Right(i32),
    Up(i32),
    Down(i32),
}

impl Direction {
    fn len(&self) -> i32 {
        match self {
            Direction::Left(x) => *x,
            Direction::Right(x) => *x,
            Direction::Up(x) => *x,
            Direction::Down(x) => *x,
        }
    }

    fn mv(&self, p: Position) -> Position {
        match self {
            Direction::Left(_) => (p.0 - 1, p.1),
            Direction::Right(_) => (p.0 + 1, p.1),
            Direction::Up(_) => (p.0, p.1 + 1),
            Direction::Down(_) => (p.0, p.1 - 1),
        }
    }

    fn string(&self) -> std::string::String {
        match self {
            Direction::Left(x) => format!("L{}", x),
            Direction::Right(x) => format!("R{}", x),
            Direction::Up(x) => format!("U{}", x),
            Direction::Down(x) => format!("D{}", x),
        }
    }
}

impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        self.string() == other.string()
    }
}

#[derive(Debug)]
struct AdventError {
    details: String,
}

impl AdventError {
    fn new(msg: &str) -> AdventError {
        AdventError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for AdventError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for AdventError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<ParseIntError> for AdventError {
    fn from(_err: ParseIntError) -> Self {
        AdventError::new("bad int parse")
    }
}

impl FromStr for Direction {
    type Err = AdventError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = s.chars().nth(0).unwrap();
        let magnitude: i32 = s[1..].parse()?;

        match direction {
            'L' => Ok(Direction::Left(magnitude)),
            'R' => Ok(Direction::Right(magnitude)),
            'U' => Ok(Direction::Up(magnitude)),
            'D' => Ok(Direction::Down(magnitude)),
            _ => Err(AdventError::new("bad direction")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use array_tool::vec::Intersect;

    #[test]
    fn parse_stuff() {
        let input = vec![(0, 1), (0, 2)];
        let input2 = vec![(0, 4), (0, 3), (0, 2)];

        assert_eq!(
            input.intersect_if(input2, |l, r| l.0 == r.0 && l.1 == r.1),
            vec![(0, 2)]
        );
    }

    #[test]
    fn parse_up() {
        let input = "U3";
        let direction: Direction = input.parse().unwrap();

        assert_eq!(direction, Direction::Up(3));
    }

    #[test]
    fn parse_down() {
        let input = "D24";
        let direction: Direction = input.parse().unwrap();

        assert_eq!(direction, Direction::Down(24));
    }

    #[test]
    fn parse_left() {
        let input = "L300";
        let direction: Direction = input.parse().unwrap();

        assert_eq!(direction, Direction::Left(300));
    }

    #[test]
    fn parse_right() {
        let input = "R72";
        let direction: Direction = input.parse().unwrap();

        assert_eq!(direction, Direction::Right(72));
    }
}
