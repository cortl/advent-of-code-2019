use math::round;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part_one();
    part_two();
}

fn part_two() {
    let filename = "src/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let fuel_needed: Vec<i32> = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            fuel_for_your_fuel(line.parse::<i32>().unwrap())
        })
        .collect();
    println!("Part 2: {}", fuel_needed.iter().sum::<i32>());
}

fn part_one() {
    let filename = "src/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let fuel_needed: Vec<i32> = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            calculate_fuel(line.parse::<i32>().unwrap())
        })
        .collect();
    println!("Part 1: {}", fuel_needed.iter().sum::<i32>());
}

pub fn calculate_fuel(mass: i32) -> i32 {
    let rounded = round::floor(f64::from(mass / 3), 0);
    rounded as i32 - 2
}

pub fn fuel_for_your_fuel(mass: i32) -> i32 {
    match calculate_fuel(mass) {
        fuel if fuel > 0 => fuel + fuel_for_your_fuel(fuel),
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_fuel_for_twelve() {
        assert_eq!(calculate_fuel(12), 2);
    }

    #[test]
    fn calculate_fuel_for_fourteen() {
        assert_eq!(calculate_fuel(14), 2);
    }

    #[test]
    fn calculate_fuel_for_1967() {
        assert_eq!(calculate_fuel(1969), 654);
    }
    #[test]
    fn calculate_fuel_for_100756() {
        assert_eq!(calculate_fuel(100756), 33583);
    }

    #[test]
    fn fuel_for_your_fuel_14() {
        assert_eq!(fuel_for_your_fuel(14), 2);
    }

    #[test]
    fn fuel_for_your_fuel_1969() {
        assert_eq!(fuel_for_your_fuel(1969), 966);
    }

    #[test]
    fn fuel_for_your_fuel_100756() {
        assert_eq!(fuel_for_your_fuel(100756), 50346);
    }
}
