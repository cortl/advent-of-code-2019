fn main() {
    println!("Part 1: {}", generate_range(307237, 769058).len());
}

fn always_increasing(num: i32) -> bool {
    let mut lowest_possible = 0;
    let mut good = true;
    for (_i, character) in num.to_string().chars().enumerate() {
        let value: i32 = character.to_digit(10).unwrap() as i32;
        if value >= lowest_possible {
            lowest_possible = value;
        } else {
            good = false;
        }
    }
    good
}

fn contains_adjacent_numbers(num: i32) -> bool {
    let mut last_num = 'x';
    let mut good: bool = false;
    for (_i, character) in num.to_string().chars().enumerate() {
        if last_num != character {
            last_num = character;
        } else {
            good = true;
        }
    }
    good
}

pub fn generate_range(start: i32, end: i32) -> Vec<i32> {
    (start + 1..end)
        .map(|x| x)
        .filter(|x| always_increasing(*x))
        .filter(|x| contains_adjacent_numbers(*x))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_numbers_between_inputs() {
        assert_eq!(generate_range(0, 20), vec![11]);
    }

    #[test]
    fn should_remove_decreased_nums() {
        let start = 4000;
        let end = 5000;

        let result = generate_range(start, end);

        assert_eq!(!result.contains(&4001), true);
        assert_eq!(result.contains(&4556), true);
    }

    #[test]
    fn should_contain_two_same_adjacent_nums() {
        let start = 4000;
        let end = 5000;

        let result = generate_range(start, end);

        assert_eq!(!result.contains(&4567), true);
        assert_eq!(result.contains(&4556), true);
    }

}
