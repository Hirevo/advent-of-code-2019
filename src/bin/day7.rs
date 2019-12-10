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
        // This constructs an iterator that yields futures where each future is a different and complete run of the feedback loop.
        let iter = (5..10).permutations(5).map(|sequence| {
            async {
                let (tx, rx) = {
                    let channels: Vec<_> = (0..=5).map(|_| sync::channel::<isize>(1)).collect();
                    for (idx, phase) in sequence.into_iter().enumerate() {
                        let (tx, rx) = channels[idx].clone();
                        tx.send(phase).await;
                        let ntx = channels[idx + 1].0.clone();
                        let interpreter = Interpreter::new(program.clone());
                        task::spawn(async move { interpreter.run_async(rx, ntx).await });
                    }
                    let tx = channels[0].0.clone();
                    let rx = channels[5].1.clone();
                    (tx, rx)
                };
                let mut output = None;
                tx.send(0).await;
                while let Some(value) = rx.recv().await {
                    output.replace(value);
                    tx.send(value).await;
                }
                output
            }
        });
        let mut max = None::<isize>;
        for fut in iter {
            let value = fut.await;
            max = match (max, value) {
                (Some(max), Some(val)) => Some(max.max(val)),
                (Some(max), None) => Some(max),
                (None, Some(val)) => Some(val),
                (None, None) => None,
            };
        }
        max.expect("not a single valid phase sequence")
    };
    println!("{0}", part2);
}
