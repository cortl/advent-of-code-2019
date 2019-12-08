// https://adventofcode.com/2019/day/5
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Result};

fn main() -> Result<()> {
    let prog = IntCodeProg::from_file("input.txt", 5)?;

    prog.take_while(|result| match result {
        OpResult::Error => panic!("Error"),
        OpResult::End => false,
        _ => true,
    })
    .for_each(|result| {
        if let OpResult::Print(val) = result {
            println!("[out] {:?}", val)
        }
    });
    Ok(())
}

type Instr = i32;

trait InstrOps {
    fn is_immediate_param(self, nth: usize) -> bool;
}

impl InstrOps for Instr {
    fn is_immediate_param(self, nth: usize) -> bool {
        (self / (10_i32.pow(nth as u32 + 1))) % 10 == 1
    }
}

#[derive(Debug, Clone)]
enum OpResult {
    Unit,
    End,
    Error,
    Print(String),
}

struct IntCodeProg {
    instrs: Vec<Instr>,
    pc: usize,
    input: i32
}

impl IntCodeProg {
    fn from_file(path: &str, input: i32) -> Result<IntCodeProg> {
        let f = File::open(path)?;
        let f = BufReader::new(f);

        let instrs: Vec<Instr> = f
            .split(b',')
            .filter_map(|token| token.ok())
            .filter_map(|token| String::from_utf8(token).ok())
            .filter_map(|num| i32::from_str_radix(&num.trim(), 10).ok())
            .collect();

        return Ok(IntCodeProg {
            instrs: instrs,
            pc: 0,
            input: input
        });
    }

    fn opcode(&self) -> i32 {
        self.instrs[self.pc] % 100
    }

    fn param(&self, nth: usize) -> i32 {
        match self.instrs[self.pc].is_immediate_param(nth) {
            true => self.instrs[self.pc + nth],
            false => self.instrs[self.instrs[self.pc + nth] as usize],
        }
    }

    fn set(&mut self, nth: usize, res: i32) -> OpResult {
        let ptr = self.instrs[self.pc + nth] as usize;
        self.instrs[ptr] = res;
        OpResult::Unit
    }

    fn eval_next(&mut self) -> Option<OpResult> {
        let (result, next_pc) = match self.opcode() {
            1 => (self.set(3, self.param(1) + self.param(2)), self.pc + 4),
            2 => (self.set(3, self.param(1) * self.param(2)), self.pc + 4),
            3 => (self.set(1, self.input), self.pc + 2),
            4 => (OpResult::Print(format!("{}", self.param(1))), self.pc + 2),
            5 => (
                OpResult::Unit,
                match self.param(1) != 0 {
                    true => self.param(2) as usize,
                    false => self.pc + 3,
                },
            ),
            6 => (
                OpResult::Unit,
                match self.param(1) == 0 {
                    true => self.param(2) as usize,
                    false => self.pc + 3,
                },
            ),
            7 => (
                self.set(
                    3,
                    match self.param(1) < self.param(2) {
                        true => 1,
                        false => 0,
                    },
                ),
                self.pc + 4,
            ),
            8 => (
                self.set(
                    3,
                    match self.param(1) == self.param(2) {
                        true => 1,
                        false => 0,
                    },
                ),
                self.pc + 4,
            ),
            99 => (OpResult::End, 0),
            _ => (OpResult::Error, 0),
        };

        match result {
            OpResult::End | OpResult::Error => (),
            _ => self.pc = next_pc,
        }

        Some(result)
    }
}

impl Iterator for IntCodeProg {
    type Item = OpResult;

    fn next(&mut self) -> Option<Self::Item> {
        self.eval_next()
    }
}
