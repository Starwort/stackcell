use std::io::{self, stdout, Read, Write};
use std::num::Wrapping;

#[cfg(feature = "u32")]
type CellSize = u32;
#[cfg(not(feature = "u32"))]
type CellSize = u8;

pub fn unhex(c: u8) -> Result<u8, String> {
    return match c {
        b'0'..=b'9' => Ok(c - b'0'),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        b'A'..=b'F' => Ok(c - b'A' + 10),
        _ => Err(format!("Invalid hex character: {}", c)),
    };
}

pub struct Computer {
    pub tape: Vec<u8>,
    pub position: usize,
    pub primary: Vec<Wrapping<CellSize>>,
    pub secondary: Vec<Wrapping<CellSize>>,
    pub io_reader: io::Stdin,
    pub running: bool,
    pub cell: Wrapping<CellSize>,
}

impl Computer {
    pub fn new(tape: &[u8]) -> Computer {
        Computer {
            tape: tape.to_vec(),
            position: 0,
            primary: vec![],
            secondary: vec![],
            io_reader: io::stdin(),
            running: true,
            cell: Wrapping(0),
        }
    }

    pub fn get(&mut self) -> Wrapping<CellSize> {
        match self.primary.pop() {
            Some(value) => value,
            None => Wrapping(0),
        }
    }

    pub fn read_tape(&mut self) -> Result<u8, String> {
        if self.position >= self.tape.len() {
            Err("Indexed out of bounds!".to_string())
        } else {
            Ok(self.tape[self.position])
        }
    }

    pub fn step(&mut self) -> Result<(), String> {
        if !self.running {
            return Err("Computer is not running!".to_string());
        }
        let instruction = self.read_tape()?;
        self.position += 1;
        match instruction {
            b'\'' => {
                let val = Wrapping(self.read_tape()? as CellSize);
                self.primary.push(val);
                self.position += 1;
            },
            b'"' => {
                while self.read_tape()? != b'"' {
                    let val = Wrapping(self.read_tape()? as CellSize);
                    self.primary.push(val);
                    self.position += 1;
                }
                self.position += 1;
            },
            b'#' => {
                let d0 = unhex(self.read_tape()?)?;
                self.position += 1;
                let d1 = unhex(self.read_tape()?)?;
                self.position += 1;
                self.primary.push(Wrapping((d0 * 16 + d1) as CellSize));
            },
            b'1'..=b'9' => {
                self.position += unhex(instruction)? as usize;
            },
            b'[' => {
                if self.get() == Wrapping(0) {
                    let mut depth = 0;
                    while depth != 0 || self.read_tape()? != b']' {
                        if self.read_tape()? == b'[' {
                            depth += 1;
                        } else if self.read_tape()? == b']' {
                            depth -= 1;
                        }
                        self.position += 1;
                    }
                    self.position += 1;
                }
            },
            b']' => {
                if self.get() != Wrapping(0) {
                    let mut depth = 0;
                    self.position -= 2;
                    while depth != 0 || self.read_tape()? != b'[' {
                        if self.read_tape()? == b']' {
                            depth += 1;
                        } else if self.read_tape()? == b'[' {
                            depth -= 1;
                        }
                        self.position -= 1;
                    }
                    self.position += 1;
                }
            },
            b'(' => {
                if self.get() != Wrapping(0) {
                    let mut depth = 0;
                    while depth != 0 || self.read_tape()? != b')' {
                        if self.read_tape()? == b'(' {
                            depth += 1;
                        } else if self.read_tape()? == b')' {
                            depth -= 1;
                        }
                        self.position += 1;
                    }
                    self.position += 1;
                }
            },
            b')' => {
                if self.get() == Wrapping(0) {
                    let mut depth = 0;
                    self.position -= 2;
                    while depth != 0 || self.read_tape()? != b'(' {
                        if self.read_tape()? == b')' {
                            depth += 1;
                        } else if self.read_tape()? == b'(' {
                            depth -= 1;
                        }
                        self.position -= 1;
                    }
                    self.position += 1;
                }
            },
            b'.' => {
                self.running = false;
            },
            b':' => {
                let val = self.get();
                self.primary.push(val);
                self.primary.push(val);
            },
            b'`' => {
                self.get();
            },
            b'{' => {
                self.cell = self.get();
            },
            b'}' => {
                self.primary.push(self.cell);
            },
            b'x' => {
                let a = self.get();
                let b = self.get();
                self.primary.push(a);
                self.primary.push(b);
            },
            b'X' => {
                std::mem::swap(&mut self.primary, &mut self.secondary);
            },
            b'!' => {
                let val = self.get();
                self.primary
                    .push(Wrapping(if val == Wrapping(0) { 1 } else { 0 }));
            },
            b'<' => {
                let left = self.get();
                let right = self.get();
                self.primary
                    .push(Wrapping(if left < right { 1 } else { 0 }));
            },
            b'>' => {
                let left = self.get();
                let right = self.get();
                self.primary
                    .push(Wrapping(if left > right { 1 } else { 0 }));
            },
            b'=' => {
                let left = self.get();
                let right = self.get();
                self.primary
                    .push(Wrapping(if left == right { 1 } else { 0 }));
            },
            b'+' => {
                let left = self.get();
                let right = self.get();
                self.primary.push(left + right);
            },
            b'-' => {
                let left = self.get();
                let right = self.get();
                self.primary.push(left - right);
            },
            b'*' => {
                let left = self.get();
                let right = self.get();
                self.primary.push(left * right);
            },
            b'/' => {
                let left = self.get();
                let right = self.get();
                self.primary.push(left / right);
            },
            b'%' => {
                let left = self.get();
                let right = self.get();
                self.primary.push(left % right);
            },
            b'^' => {
                let left = self.get();
                let right = self.get();
                self.primary.push(left ^ right);
            },
            b'&' => {
                let left = self.get();
                let right = self.get();
                self.primary.push(left & right);
            },
            b'|' => {
                let left = self.get();
                let right = self.get();
                self.primary.push(left | right);
            },
            b'~' => {
                let val = self.get();
                self.primary.push(!val);
            },
            b'?' => {
                let val = self.get();
                if val == Wrapping(0) {
                    self.position += 1;
                }
            },
            b';' => {
                print!("{}", self.get().0 as u8 as char);
                stdout().flush().unwrap();
            },
            b'@' => {
                let mut buf = [0u8];
                let result =
                    self.io_reader.read(&mut buf).map_err(|e| e.to_string())?;
                if result == 0 {
                    self.primary.push(Wrapping(0));
                } else {
                    self.primary.push(Wrapping(buf[0] as CellSize));
                }
            },
            _ => return Err(format!("Unknown instruction: {:?}", instruction as char)),
        }
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), String> {
        while self.position < self.tape.len() && self.running {
            self.step()?;
        }
        Ok(())
    }
}
