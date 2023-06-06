use std::io::{stdout, Error, ErrorKind};
use std::num::Wrapping;

use crossterm::cursor::MoveTo;
use crossterm::event::Event::Key;
use crossterm::event::{read, KeyCode, KeyEvent, KeyModifiers};
use crossterm::execute;

use crate::constants::{INPUT_BOX, TEMPLATE_BOTTOM};

#[cfg(feature = "u32")]
type CellSize = u32;
#[cfg(not(feature = "u32"))]
pub(crate) type CellSize = u8;

pub fn unhex(c: u8) -> Result<u8, Error> {
    match c {
        b'0'..=b'9' => Ok(c - b'0'),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        b'A'..=b'F' => Ok(c - b'A' + 10),
        _ => {
            Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Invalid hex character: {c}"),
            ))
        },
    }
}

pub struct Computer {
    pub tape: Vec<u8>,
    pub position: usize,
    pub primary: Vec<Wrapping<CellSize>>,
    pub secondary: Vec<Wrapping<CellSize>>,
    pub running: bool,
    pub cell: Wrapping<CellSize>,
    pub has_found_eof: bool,
}

impl Computer {
    pub fn new(tape: &str) -> Computer {
        Computer {
            tape: tape.bytes().collect(),
            position: 0,
            primary: vec![],
            secondary: vec![],
            running: true,
            cell: Wrapping(0),
            has_found_eof: false,
        }
    }

    pub fn get(&mut self) -> Wrapping<CellSize> {
        match self.primary.pop() {
            Some(value) => value,
            None => Wrapping(0),
        }
    }

    pub fn read_tape(&mut self) -> Result<u8, Error> {
        if self.position >= self.tape.len() {
            Err(Error::new(ErrorKind::Other, "Indexed out of bounds!"))
        } else {
            Ok(self.tape[self.position])
        }
    }

    fn find_matching_open(&mut self, open: u8, close: u8) -> Result<(), Error> {
        let mut depth = 0;
        self.position -= 2;
        while depth != 0 || self.read_tape()? != open {
            if self.read_tape()? == close {
                depth += 1;
            } else if self.read_tape()? == open {
                depth -= 1;
            }
            if self.position == 0 {
                return Err(Error::new(ErrorKind::Other, "No matching open!"));
            }
            self.position -= 1;
        }
        self.position += 1;
        Ok(())
    }

    fn find_matching_close(&mut self, open: u8, close: u8) -> Result<(), Error> {
        let mut depth = 0;
        while depth != 0 || self.read_tape()? != close {
            if self.read_tape()? == open {
                depth += 1;
            } else if self.read_tape()? == close {
                depth -= 1;
            }
            self.position += 1;
        }
        self.position += 1;
        Ok(())
    }

    fn do_op(
        &mut self,
        op: fn(Wrapping<CellSize>, Wrapping<CellSize>) -> Wrapping<CellSize>,
    ) {
        let left = self.get();
        let right = self.get();
        self.primary.push(op(left, right));
    }

    // essentially a big match statement, not sure how to refactor
    // not really a big deal that it's 40loc over imo
    #[allow(clippy::too_many_lines)]
    pub fn step(&mut self, output: &mut String) -> Result<(), Error> {
        if !self.running {
            return Err(Error::new(ErrorKind::Other, "Computer is not running!"));
        } else if self.position >= self.tape.len() {
            // this is fine, just halt
            self.running = false;
            return Ok(());
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
                    self.find_matching_close(b'[', b']')?;
                }
            },
            b']' => {
                if self.get() != Wrapping(0) {
                    self.find_matching_open(b'[', b']')?;
                }
            },
            b'(' => {
                if self.get() != Wrapping(0) {
                    self.find_matching_close(b'(', b')')?;
                }
            },
            b')' => {
                if self.get() == Wrapping(0) {
                    self.find_matching_open(b'(', b')')?;
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
                self.primary.push(Wrapping(u8::from(val == Wrapping(0))));
            },
            b'<' => {
                self.do_op(|left, right| Wrapping(u8::from(left < right)));
            },
            b'>' => {
                self.do_op(|left, right| Wrapping(u8::from(left > right)));
            },
            b'=' => {
                self.do_op(|left, right| Wrapping(u8::from(left == right)));
            },
            b'+' => {
                self.do_op(|left, right| left + right);
            },
            b'-' => {
                self.do_op(|left, right| left - right);
            },
            b'*' => {
                self.do_op(|left, right| left * right);
            },
            b'/' => {
                self.do_op(|left, right| left / right);
            },
            b'%' => {
                self.do_op(|left, right| left % right);
            },
            b'^' => {
                self.do_op(|left, right| left ^ right);
            },
            b'&' => {
                self.do_op(|left, right| left & right);
            },
            b'|' => {
                self.do_op(|left, right| left | right);
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
                // when CellSize = u8, this triggers unnecessary-cast; when it is u32,
                // it triggers cast-possible-truncation.
                // unnecessary-cast is a false positive, and
                // cast-possible-truncation is intended behaviour.
                #[allow(clippy::cast_possible_truncation, clippy::unnecessary_cast)]
                output.push(self.get().0 as u8 as char);
            },
            b'@' => {
                let val = Wrapping(self.get_input()?);
                self.primary.push(val);
            },
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("Unknown instruction: {:?}", instruction as char),
                ))
            },
        }
        Ok(())
    }

    pub fn get_input(&mut self) -> Result<CellSize, Error> {
        if self.has_found_eof {
            return Ok(0);
        }
        loop {
            execute!(std::io::stdout(), crossterm::cursor::MoveTo(0, 17))?;
            print!("{INPUT_BOX}");
            match read()? {
                Key(KeyEvent {
                    code: KeyCode::Char('D' | 'd'),
                    modifiers: KeyModifiers::CONTROL,
                }) => {
                    execute!(stdout(), MoveTo(0, 17))?;
                    print!("{TEMPLATE_BOTTOM}");
                    self.has_found_eof = true;
                    return Ok(0);
                },
                Key(KeyEvent {
                    code: KeyCode::Char(c),
                    ..
                }) => {
                    execute!(stdout(), MoveTo(0, 17))?;
                    print!("{TEMPLATE_BOTTOM}");
                    return Ok(c as CellSize);
                },
                Key(KeyEvent {
                    code: KeyCode::Enter,
                    ..
                }) => {
                    execute!(stdout(), MoveTo(0, 17))?;
                    print!("{TEMPLATE_BOTTOM}");
                    return Ok(b'\n' as CellSize);
                },
                Key(KeyEvent {
                    code: KeyCode::Esc,
                    ..
                }) => {
                    execute!(stdout(), MoveTo(0, 17))?;
                    print!("{TEMPLATE_BOTTOM}");
                    self.running = false;
                    return Err(Error::new(ErrorKind::Other, "User exit"));
                },
                _ => (),
            }
        }
    }

    pub fn run(&mut self, output: &mut String) -> Result<(), Error> {
        while self.position < self.tape.len() && self.running {
            self.step(output)?;
        }
        Ok(())
    }
}
