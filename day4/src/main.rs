use regex::Regex;
use regex::RegexSet;

fn main() {
    println!("Part 2: {}", generate_range(307237, 769058).len());
}

fn always_increasing(num: i32) -> bool {
    let number = num.to_string();
    for i in 0..5 {
        let current_value: i32 = (number.as_bytes()[i] as char).to_digit(10).unwrap() as i32;
        let next_value: i32 = (number.as_bytes()[i + 1] as char).to_digit(10).unwrap() as i32;
        if next_value < current_value {
            return false;
        }
    }
    true
}

fn contains_adjacent_numbers(num: i32) -> bool {
    let reg_set = RegexSet::new(&[
        r"00++", r"11+", r"22+", r"33+", r"44+", r"55+", r"66+", r"77+", r"88+", r"99+",
    ])
    .unwrap();
    let number = num.to_string();
    let matches: Vec<_> = reg_set
        .matches(&number)
        .into_iter()
        .map(|pattern_i| {
            let re = Regex::new(&reg_set.patterns()[pattern_i]).unwrap();
            re.find(&number).unwrap().as_str().len()
        })
        .collect();
    matches.contains(&2usize)
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

        assert_eq!(result.contains(&4001), false);
        assert_eq!(result.contains(&4556), true);
    }

    #[test]
    fn should_contain_two_same_adjacent_nums() {
        let start = 4000;
        let end = 5000;

        let result = generate_range(start, end);

        assert_eq!(result.contains(&4567), false);
        assert_eq!(result.contains(&4556), true);
    }

    #[test]
    fn should_contain_two_same_and_no_moreadjacent_nums() {
        let result: Vec<i32> = generate_range(0, 2000000)
            .iter()
            .map(|x| *x)
            .filter(|x| contains_adjacent_numbers(*x))
            .collect();

        assert_eq!(result.contains(&112233), true);
        assert_eq!(result.contains(&123444), false);
        assert_eq!(result.contains(&111122), true);
    }

}
