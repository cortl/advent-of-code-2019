use math::round;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let fuel_needed: Vec<u32> = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            calculate_fuel(line.parse::<u32>().unwrap())
        })
        .collect();
        
    println!("Total fuel required, {:?}", fuel_needed.iter().sum::<u32>());
}

pub fn calculate_fuel(mass: u32) -> u32 {
    let rounded = round::floor(f64::from(mass / 3), 0);
    rounded as u32 - 2
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
}
