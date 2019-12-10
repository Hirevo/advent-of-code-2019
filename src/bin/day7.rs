use std::collections::VecDeque;
use std::convert::TryFrom;

use async_std::sync;
use async_std::task;
use itertools::Itertools;

pub static INPUT: &str = include_str!("../../inputs/day7.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Opcode {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    EqualTo,
    Halt,
}

impl TryFrom<usize> for Opcode {
    type Error = ();
    fn try_from(num: usize) -> Result<Opcode, Self::Error> {
        match num {
            1 => Ok(Opcode::Add),
            2 => Ok(Opcode::Multiply),
            3 => Ok(Opcode::Input),
            4 => Ok(Opcode::Output),
            5 => Ok(Opcode::JumpIfTrue),
            6 => Ok(Opcode::JumpIfFalse),
            7 => Ok(Opcode::LessThan),
            8 => Ok(Opcode::EqualTo),
            99 => Ok(Opcode::Halt),
            _ => Err(()),
        }
    }
}

pub struct Interpreter {
    pub program: Vec<isize>,
}

impl Interpreter {
    pub fn new(program: Vec<isize>) -> Interpreter {
        Interpreter { program }
    }

    pub fn run(&self, mut inputs: impl Iterator<Item = isize>) -> Option<isize> {
        let mut memory = self.program.clone();

        let mut output = None;
        let mut cursor = 0;
        loop {
            let mut instr = memory[cursor];
            let opcode = instr % 100;
            instr /= 100;
            let opcode = Opcode::try_from(opcode as usize).ok()?;
            match opcode {
                Opcode::Add | Opcode::Multiply | Opcode::LessThan | Opcode::EqualTo => {
                    let arg1 = memory[cursor + 1];
                    let arg2 = memory[cursor + 2];
                    let arg3 = memory[cursor + 3];

                    let mode1 = instr % 10;
                    instr /= 10;
                    let mode2 = instr % 10;
                    instr /= 10;
                    let mode3 = instr % 10;

                    let val1 = match mode1 {
                        0 => memory[arg1 as usize],
                        1 => arg1,
                        _ => panic!("invalid parameter mode"),
                    };
                    let val2 = match mode2 {
                        0 => memory[arg2 as usize],
                        1 => arg2,
                        _ => panic!("invalid parameter mode"),
                    };

                    if mode3 != 0 {
                        panic!("invalid parameter mode");
                    }

                    memory[arg3 as usize] = match opcode {
                        Opcode::Add => val1 + val2,
                        Opcode::Multiply => val1 * val2,
                        Opcode::LessThan => {
                            if val1 < val2 {
                                1
                            } else {
                                0
                            }
                        }
                        Opcode::EqualTo => {
                            if val1 == val2 {
                                1
                            } else {
                                0
                            }
                        }
                        _ => return None,
                    };

                    cursor += 4;
                }
                Opcode::Input => {
                    let arg1 = memory[cursor + 1];

                    if instr % 10 != 0 {
                        return None;
                    }

                    let value = inputs.next()?;

                    memory[arg1 as usize] = value;

                    cursor += 2;
                }
                Opcode::Output => {
                    let arg1 = memory[cursor + 1];

                    let val1 = match instr % 10 {
                        0 => memory[arg1 as usize],
                        1 => arg1,
                        _ => panic!("invalid parameter mode"),
                    };

                    output.replace(val1);

                    cursor += 2;
                }
                Opcode::JumpIfTrue => {
                    let arg1 = memory[cursor + 1];
                    let arg2 = memory[cursor + 2];

                    let mode1 = instr % 10;
                    instr /= 10;
                    let mode2 = instr % 10;

                    let val1 = match mode1 {
                        0 => memory[arg1 as usize],
                        1 => arg1,
                        _ => panic!("invalid parameter mode"),
                    };
                    let val2 = match mode2 {
                        0 => memory[arg2 as usize],
                        1 => arg2,
                        _ => panic!("invalid parameter mode"),
                    };

                    if val1 != 0 {
                        cursor = val2 as usize;
                    } else {
                        cursor += 3;
                    }
                }
                Opcode::JumpIfFalse => {
                    let arg1 = memory[cursor + 1];
                    let arg2 = memory[cursor + 2];

                    let mode1 = instr % 10;
                    instr /= 10;
                    let mode2 = instr % 10;

                    let val1 = match mode1 {
                        0 => memory[arg1 as usize],
                        1 => arg1,
                        _ => panic!("invalid parameter mode"),
                    };
                    let val2 = match mode2 {
                        0 => memory[arg2 as usize],
                        1 => arg2,
                        _ => panic!("invalid parameter mode"),
                    };

                    if val1 == 0 {
                        cursor = val2 as usize;
                    } else {
                        cursor += 3;
                    }
                }
                Opcode::Halt => break,
            }
        }

        output
    }

    pub async fn run_async(
        &self,
        input: sync::Receiver<isize>,
        output: sync::Sender<isize>,
    ) -> Option<()> {
        let mut memory = self.program.clone();

        let mut cursor = 0;
        loop {
            let mut instr = memory[cursor];
            let opcode = instr % 100;
            instr /= 100;
            let opcode = Opcode::try_from(opcode as usize).ok()?;
            match opcode {
                Opcode::Add | Opcode::Multiply | Opcode::LessThan | Opcode::EqualTo => {
                    let arg1 = memory[cursor + 1];
                    let arg2 = memory[cursor + 2];
                    let arg3 = memory[cursor + 3];

                    let mode1 = instr % 10;
                    instr /= 10;
                    let mode2 = instr % 10;
                    instr /= 10;
                    let mode3 = instr % 10;

                    let val1 = match mode1 {
                        0 => memory[arg1 as usize],
                        1 => arg1,
                        _ => panic!("invalid parameter mode"),
                    };
                    let val2 = match mode2 {
                        0 => memory[arg2 as usize],
                        1 => arg2,
                        _ => panic!("invalid parameter mode"),
                    };

                    if mode3 != 0 {
                        panic!("invalid parameter mode");
                    }

                    memory[arg3 as usize] = match opcode {
                        Opcode::Add => val1 + val2,
                        Opcode::Multiply => val1 * val2,
                        Opcode::LessThan => {
                            if val1 < val2 {
                                1
                            } else {
                                0
                            }
                        }
                        Opcode::EqualTo => {
                            if val1 == val2 {
                                1
                            } else {
                                0
                            }
                        }
                        _ => return None,
                    };

                    cursor += 4;
                }
                Opcode::Input => {
                    let arg1 = memory[cursor + 1];

                    if instr % 10 != 0 {
                        return None;
                    }

                    let value = input.recv().await?;

                    memory[arg1 as usize] = value;

                    cursor += 2;
                }
                Opcode::Output => {
                    let arg1 = memory[cursor + 1];

                    let val1 = match instr % 10 {
                        0 => memory[arg1 as usize],
                        1 => arg1,
                        _ => panic!("invalid parameter mode"),
                    };

                    output.send(val1).await;

                    cursor += 2;
                }
                Opcode::JumpIfTrue => {
                    let arg1 = memory[cursor + 1];
                    let arg2 = memory[cursor + 2];

                    let mode1 = instr % 10;
                    instr /= 10;
                    let mode2 = instr % 10;

                    let val1 = match mode1 {
                        0 => memory[arg1 as usize],
                        1 => arg1,
                        _ => panic!("invalid parameter mode"),
                    };
                    let val2 = match mode2 {
                        0 => memory[arg2 as usize],
                        1 => arg2,
                        _ => panic!("invalid parameter mode"),
                    };

                    if val1 != 0 {
                        cursor = val2 as usize;
                    } else {
                        cursor += 3;
                    }
                }
                Opcode::JumpIfFalse => {
                    let arg1 = memory[cursor + 1];
                    let arg2 = memory[cursor + 2];

                    let mode1 = instr % 10;
                    instr /= 10;
                    let mode2 = instr % 10;

                    let val1 = match mode1 {
                        0 => memory[arg1 as usize],
                        1 => arg1,
                        _ => panic!("invalid parameter mode"),
                    };
                    let val2 = match mode2 {
                        0 => memory[arg2 as usize],
                        1 => arg2,
                        _ => panic!("invalid parameter mode"),
                    };

                    if val1 == 0 {
                        cursor = val2 as usize;
                    } else {
                        cursor += 3;
                    }
                }
                Opcode::Halt => break,
            }
        }

        Some(())
    }
}

#[async_std::main]
async fn main() {
    let program: Vec<isize> = INPUT
        .trim()
        .split(',')
        .flat_map(|chunk| chunk.parse().ok())
        .collect();

    let interpreters: Vec<_> = (0..5).map(|_| Interpreter::new(program.clone())).collect();

    let part1 = (0..5)
        .permutations(5)
        .flat_map(|sequence| {
            sequence
                .into_iter()
                .zip(interpreters.iter())
                .fold(Some(0), |acc, (phase, interpreter)| {
                    interpreter.run([phase, acc?].iter().copied())
                })
                .into_iter()
        })
        .max()
        .expect("not a single valid phase sequence");
    println!("{0}", part1);

    let part2 = {
        let mut max = None::<isize>;
        for sequence in (5..10).permutations(5) {
            let (tx, rx) = {
                // create channels
                let (mut txs, mut rxs) = (0..=5).fold(
                    (VecDeque::with_capacity(6), VecDeque::with_capacity(6)),
                    |(mut txs, mut rxs), _| {
                        let (tx, rx) = sync::channel::<isize>(1);
                        txs.push_back(tx);
                        rxs.push_back(rx);
                        (txs, rxs)
                    },
                );

                // initialize phase sequences (except last one, `Iterator::zip` takes care of that already)
                for (tx, phase) in txs.iter().zip(sequence.into_iter()) {
                    tx.send(phase).await;
                }

                // extract pipeline handles
                let tx = txs.pop_front().expect("no channels ??");
                let rx = rxs.pop_back().expect("no channels ??");

                // spawn tasks
                for (tx, rx) in txs.into_iter().zip(rxs.into_iter()) {
                    let interpreter = Interpreter::new(program.clone());
                    task::spawn(async move { interpreter.run_async(rx, tx).await });
                }

                (tx, rx)
            };

            // send first value and wait until last output
            tx.send(0).await;
            let mut output = None;
            while let Some(value) = rx.recv().await {
                output.replace(value);
                tx.send(value).await;
            }

            // save if highest output seen yet
            max = match (max, output) {
                (Some(max), Some(output)) => Some(max.max(output)),
                (Some(max), None) => Some(max),
                (None, Some(val)) => Some(val),
                (None, None) => None,
            };
        }

        max.expect("not a single valid phase sequence")
    };
    println!("{0}", part2);
}
