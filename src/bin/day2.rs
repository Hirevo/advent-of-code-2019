use rayon::prelude::*;

use aoc_2019::Error;

pub static INPUT: &str = include_str!("../../inputs/day2.txt");

pub struct Interpreter {
    pub program: Vec<usize>,
}

impl Interpreter {
    pub fn new(program: Vec<usize>) -> Interpreter {
        Interpreter { program }
    }

    /// Run the program with the given noun and verb.
    /// This version is safe: any invalid memory accesses will just return `None` instead of panicking.
    pub fn run_safe(&self, noun: usize, verb: usize) -> Option<usize> {
        let mut memory = self.program.clone();

        *(memory.get_mut(1)?) = noun;
        *(memory.get_mut(2)?) = verb;

        let mut cursor = 0;
        loop {
            let opcode = *(memory.get(cursor)?);
            match opcode {
                1 | 2 => {
                    let arg1 = *(memory.get(cursor + 1)?);
                    let arg2 = *(memory.get(cursor + 2)?);
                    let arg3 = *(memory.get(cursor + 3)?);

                    let val1 = *(memory.get(arg1)?);
                    let val2 = *(memory.get(arg2)?);

                    *(memory.get_mut(arg3)?) = match opcode {
                        1 => val1 + val2,
                        2 => val1 * val2,
                        _ => unreachable!(),
                    };

                    cursor += 4;
                }
                99 => break,
                _ => return None,
            }
        }

        Some(memory[0])
    }

    /// Run the program with the given noun and verb.
    /// This version will panic if an invalid memory access is performed.
    pub fn run(&self, noun: usize, verb: usize) -> Option<usize> {
        let mut memory = self.program.clone();

        memory[1] = noun;
        memory[2] = verb;

        let mut cursor = 0;
        loop {
            let opcode = memory[cursor];
            match opcode {
                1 | 2 => {
                    let arg1 = memory[cursor + 1];
                    let arg2 = memory[cursor + 2];
                    let arg3 = memory[cursor + 3];

                    let val1 = memory[arg1];
                    let val2 = memory[arg2];

                    memory[arg3] = match opcode {
                        1 => val1 + val2,
                        2 => val1 * val2,
                        _ => unreachable!(),
                    };

                    cursor += 4;
                }
                99 => break,
                _ => return None,
            }
        }

        Some(memory[0])
    }
}

fn main() -> Result<(), Error> {
    let program = INPUT
        .split(',')
        .flat_map(|chunk| chunk.parse().ok())
        .collect();
    let interpreter = Interpreter::new(program);

    let part1 = interpreter.run(12, 2).expect("invalid input program");
    println!("{0}", part1);

    let part2 = {
        let inputs: Vec<(usize, usize)> = (0..=99)
            .flat_map(|i| (0..=99).map(move |j| (i, j)))
            .collect();
        let found = inputs.into_par_iter().find_any(|(noun, verb)| {
            interpreter
                .run(*noun, *verb)
                .map_or(false, |ret| ret == 19690720)
        });
        dbg!(found).map(|(noun, verb)| 100 * noun + verb)
    };
    println!(
        "{0}",
        part2.expect("couldn't find any matching noun and verb")
    );

    Ok(())
}
