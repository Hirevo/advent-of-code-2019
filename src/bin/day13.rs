use std::collections::HashMap;
use std::convert::TryFrom;

use async_std::sync;
use async_std::task;

pub static INPUT: &str = include_str!("../../inputs/day13.txt");

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
    AdjustRelativeBase,
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
            9 => Ok(Opcode::AdjustRelativeBase),
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

    pub async fn run_async(
        &self,
        input: sync::Receiver<isize>,
        output: sync::Sender<isize>,
    ) -> Option<isize> {
        let mut memory = self.program.clone();
        memory.resize(999999, 0);

        let mut relative_base = 0;
        let mut cursor = 0;
        let mut cached = None;
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
                        2 => memory[(relative_base + arg1) as usize],
                        _ => panic!("invalid parameter mode"),
                    };
                    let val2 = match mode2 {
                        0 => memory[arg2 as usize],
                        1 => arg2,
                        2 => memory[(relative_base + arg2) as usize],
                        _ => panic!("invalid parameter mode"),
                    };

                    let place = match mode3 {
                        0 => &mut memory[arg3 as usize],
                        2 => &mut memory[(relative_base + arg3) as usize],
                        _ => panic!("invalid parameter mode"),
                    };
                    *place = match opcode {
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

                    let mode1 = instr % 10;

                    let value = match cached {
                        Some(cached) => cached,
                        None => {
                            let value = input.recv().await?;
                            cached.replace(value);
                            value
                        }
                    };

                    let place = match mode1 {
                        0 => &mut memory[arg1 as usize],
                        2 => &mut memory[(relative_base + arg1) as usize],
                        _ => panic!("invalid parameter mode"),
                    };
                    *place = value;

                    cursor += 2;
                }
                Opcode::Output => {
                    let arg1 = memory[cursor + 1];

                    let val1 = match instr % 10 {
                        0 => memory[arg1 as usize],
                        1 => arg1,
                        2 => memory[(relative_base + arg1) as usize],
                        _ => panic!("invalid parameter mode"),
                    };

                    output.send(val1).await;
                    let _ = cached.take();

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
                        2 => memory[(relative_base + arg1) as usize],
                        _ => panic!("invalid parameter mode"),
                    };
                    let val2 = match mode2 {
                        0 => memory[arg2 as usize],
                        1 => arg2,
                        2 => memory[(relative_base + arg2) as usize],
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
                        2 => memory[(relative_base + arg1) as usize],
                        _ => panic!("invalid parameter mode"),
                    };
                    let val2 = match mode2 {
                        0 => memory[arg2 as usize],
                        1 => arg2,
                        2 => memory[(relative_base + arg2) as usize],
                        _ => panic!("invalid parameter mode"),
                    };

                    if val1 == 0 {
                        cursor = val2 as usize;
                    } else {
                        cursor += 3;
                    }
                }
                Opcode::AdjustRelativeBase => {
                    let arg1 = memory[cursor + 1];

                    let mode1 = instr % 10;

                    let val1 = match mode1 {
                        0 => memory[arg1 as usize],
                        1 => arg1,
                        2 => memory[(relative_base + arg1) as usize],
                        _ => panic!("invalid parameter mode"),
                    };

                    relative_base += val1;

                    cursor += 2;
                }
                Opcode::Halt => break,
            }
        }

        Some(memory[0])
    }
}

#[async_std::main]
async fn main() {
    let program: Vec<isize> = INPUT
        .trim()
        .split(',')
        .flat_map(|chunk| chunk.parse().ok())
        .collect();

    let interpreter = Interpreter::new(program.clone());

    let (_, rx1) = sync::channel::<isize>(1);
    let (tx2, rx2) = sync::channel::<isize>(1);

    task::spawn(async move {
        interpreter
            .run_async(rx1, tx2)
            .await
            .expect("invalid program")
    });

    let mut image = HashMap::new();
    while let Some((x, y, val)) = {
        let x = rx2.recv().await;
        let y = rx2.recv().await;
        let val = rx2.recv().await;
        x.and_then(move |x| y.map(|y| (x, y)))
            .and_then(|(x, y)| val.map(move |val| (x, y, val)))
    } {
        image.insert((x, y), val);
    }

    let part1 = image.iter().filter(|&(_, &v)| v == 2).count();
    println!("part 1: {0}", part1);

    // For part 2, we edited the input program to have a full row of pads at the bottom.
    // This guarantees us to destroy all blocks eventually, so we keep sending neutral joystick input to the program.

    let interpreter = Interpreter::new({
        let mut program = program.clone();
        program[0] = 2;
        program
    });

    let (tx1, rx1) = sync::channel::<isize>(1);
    let (tx2, rx2) = sync::channel::<isize>(1);

    task::spawn(async move {
        interpreter
            .run_async(rx1, tx2)
            .await
            .expect("invalid program")
    });
    task::spawn(async move {
        loop {
            tx1.send(0).await;
        }
    });

    let mut part2 = 0;
    while let Some(val) = {
        rx2.recv().await;
        rx2.recv().await;
        rx2.recv().await
    } {
        part2 = val;
    }

    println!("part 2: {0}", part2);
}
