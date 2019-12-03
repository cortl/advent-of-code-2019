// https://adventofcode.com/2019/day/2

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let input = &mut vec![
        1, 12, 2, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 6, 19, 1, 5, 19, 23, 1, 23, 6, 27,
        1, 5, 27, 31, 1, 31, 6, 35, 1, 9, 35, 39, 2, 10, 39, 43, 1, 43, 6, 47, 2, 6, 47, 51, 1, 5,
        51, 55, 1, 55, 13, 59, 1, 59, 10, 63, 2, 10, 63, 67, 1, 9, 67, 71, 2, 6, 71, 75, 1, 5, 75,
        79, 2, 79, 13, 83, 1, 83, 5, 87, 1, 87, 9, 91, 1, 5, 91, 95, 1, 5, 95, 99, 1, 99, 13, 103,
        1, 10, 103, 107, 1, 107, 9, 111, 1, 6, 111, 115, 2, 115, 13, 119, 1, 10, 119, 123, 2, 123,
        6, 127, 1, 5, 127, 131, 1, 5, 131, 135, 1, 135, 6, 139, 2, 139, 10, 143, 2, 143, 9, 147, 1,
        147, 6, 151, 1, 151, 13, 155, 2, 155, 9, 159, 1, 6, 159, 163, 1, 5, 163, 167, 1, 5, 167,
        171, 1, 10, 171, 175, 1, 13, 175, 179, 1, 179, 2, 183, 1, 9, 183, 0, 99, 2, 14, 0, 0,
    ];
    println!("Part 1: {}", process_program(input)[0]);
}

fn part_two() {
    let input = &mut vec![
        1, 12, 2, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 6, 19, 1, 5, 19, 23, 1, 23, 6, 27,
        1, 5, 27, 31, 1, 31, 6, 35, 1, 9, 35, 39, 2, 10, 39, 43, 1, 43, 6, 47, 2, 6, 47, 51, 1, 5,
        51, 55, 1, 55, 13, 59, 1, 59, 10, 63, 2, 10, 63, 67, 1, 9, 67, 71, 2, 6, 71, 75, 1, 5, 75,
        79, 2, 79, 13, 83, 1, 83, 5, 87, 1, 87, 9, 91, 1, 5, 91, 95, 1, 5, 95, 99, 1, 99, 13, 103,
        1, 10, 103, 107, 1, 107, 9, 111, 1, 6, 111, 115, 2, 115, 13, 119, 1, 10, 119, 123, 2, 123,
        6, 127, 1, 5, 127, 131, 1, 5, 131, 135, 1, 135, 6, 139, 2, 139, 10, 143, 2, 143, 9, 147, 1,
        147, 6, 151, 1, 151, 13, 155, 2, 155, 9, 159, 1, 6, 159, 163, 1, 5, 163, 167, 1, 5, 167,
        171, 1, 10, 171, 175, 1, 13, 175, 179, 1, 179, 2, 183, 1, 9, 183, 0, 99, 2, 14, 0, 0,
    ];

    let goal = 19690720;
    let mut resulting_noun = 0;
    let mut resulting_verb = 0;
    for noun in 0..99usize {
        for verb in 0..99usize {
            let new_input = &mut input.clone();
            new_input[1] = noun as u32;
            new_input[2] = verb as u32;
            if process_program(new_input)[0usize] == goal {
                resulting_noun = noun;
                resulting_verb = verb;
            }
        }
    }
    println!("Part 2: Noun: {}, Verb: {}, Result: {}", resulting_noun, resulting_verb, 
    100 * resulting_noun + resulting_verb);
}

pub fn process_program(program: &mut Vec<u32>) -> &mut Vec<u32> {
    let mut i = 0usize;
    loop {
        let opcode = *program.get(i).unwrap();
        match opcode {
            1 => {
                let operand1 = program[program[i + 1] as usize];
                let operand2 = program[program[i + 2] as usize];
                let store_index = program[i + 3];
                program[store_index as usize] = operand1 + operand2;
            }
            2 => {
                let operand1 = program[program[i + 1] as usize];
                let operand2 = program[program[i + 2] as usize];
                let store_index = program[i + 3];
                program[store_index as usize] = operand1 * operand2;
            }
            99 => break,
            _ => unreachable!(),
        }
        i += 4;
    }
    program
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_program_for_simple_one_opcode() {
        let program: &mut Vec<u32> = &mut vec![1, 0, 0, 0, 99];
        assert_eq!(*process_program(program), vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn run_program_for_simple_two_opcode() {
        let program: &mut Vec<u32> = &mut vec![2, 3, 0, 3, 99];
        assert_eq!(*process_program(program), vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn run_program_for_extra_two_opcode() {
        let program: &mut Vec<u32> = &mut vec![2, 4, 4, 5, 99, 0];
        assert_eq!(*process_program(program), vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn run_program_for_multiple_opcode() {
        let program: &mut Vec<u32> = &mut vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        assert_eq!(*process_program(program), vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn run_program_for_multiple_opcode_2() {
        let program: &mut Vec<u32> = &mut vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        assert_eq!(
            *process_program(program),
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
    }
}
