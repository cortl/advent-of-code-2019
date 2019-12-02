use math::round;

fn main() {}

pub fn calculate_fuel(mass: i32) -> i32 {
    let rounded = round::floor(f64::from(mass / 3), 0);
    println!("{}", rounded);
    rounded as i32 - 2
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
