use std::cmp::{max, min};
use std::io::{stdout, Error, ErrorKind, Write};
use std::num::Wrapping;

use crossterm::event::read;
use crossterm::terminal::{self, disable_raw_mode, enable_raw_mode, ClearType};
use crossterm::{cursor, event, execute, style};

mod constants;
mod types;
use constants::{CLEAR_LINE, ENTRY_BOX, TEMPLATE, TEMPLATE_BOTTOM};
use types::CellSize;

fn run_app() -> Result<(), Error> {
    let mut stdout = stdout();
    execute!(
        stdout,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )?;
    write!(stdout, "{TEMPLATE}")?;
    let mut program = String::new();
    let mut filename = "No file".to_string();
    let mut computer = types::Computer::new(&program);
    let mut output = String::new();
    let mut running_as_fast_as_possible = false;
    loop {
        print_info(&mut stdout, &computer, &filename, &output)?;
        #[allow(clippy::redundant_else)]
        if running_as_fast_as_possible && computer.running {
            match computer.step(&mut output) {
                Ok(()) => (),
                Err(e) => {
                    output.push_str(&format!("Error: {e}"));
                    computer.running = false;
                    running_as_fast_as_possible = false;
                },
            }
            continue;
        } else {
            running_as_fast_as_possible = false;
        }

        match read()? {
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Esc,
                ..
            }) => return Ok(()),
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::F(2),
                ..
            }) => {
                if let Err(e) = get_string(&mut stdout, &mut filename) {
                    output.push_str(&e.to_string());
                    continue;
                }
                program = match std::fs::read_to_string(&filename) {
                    Ok(s) => s,
                    Err(e) => {
                        output.push_str(&e.to_string());
                        continue;
                    },
                };
                computer = types::Computer::new(&program);
            },
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::F(3),
                ..
            }) => {
                filename = "<stdin>".to_string();
                if let Err(e) = get_string(&mut stdout, &mut program) {
                    output.push_str(&e.to_string());
                    continue;
                }
                computer = types::Computer::new(&program);
            },
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::F(5),
                ..
            }) => {
                match computer.step(&mut output) {
                    Ok(()) => (),
                    Err(e) => {
                        output.push_str(&format!("Error: {e}"));
                        computer.running = false;
                    },
                }
            },
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::F(9),
                ..
            }) => {
                output.clear();
                computer = types::Computer::new(&program);
            },
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::F(10),
                ..
            }) => {
                running_as_fast_as_possible = true;
            },
            _ => (),
        }
    }
}

fn get_string(stdout: &mut std::io::Stdout, out: &mut String) -> Result<(), Error> {
    execute!(stdout, cursor::MoveTo(0, 17))?;
    write!(stdout, "{ENTRY_BOX}")?;
    out.clear();
    loop {
        execute!(stdout, cursor::MoveTo(2, 18))?;
        let spaces = if out.len() > 98 { 0 } else { 98 - out.len() };
        let out_start = if out.len() > 98 { out.len() - 96 } else { 0 };
        let render_arrow = out.len() > 98;
        write!(
            stdout,
            "{}{}_{}",
            if render_arrow { "< " } else { "" },
            &out[out_start..],
            &CLEAR_LINE[..spaces],
        )?;
        stdout.flush()?;
        match read()? {
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Esc,
                ..
            }) => {
                execute!(stdout, cursor::MoveTo(0, 17))?;
                write!(stdout, "{TEMPLATE_BOTTOM}")?;
                execute!(stdout, terminal::Clear(ClearType::FromCursorDown))?;
                return Err(Error::new(ErrorKind::Other, "Cancelled"));
            },
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Enter,
                ..
            }) => {
                execute!(stdout, cursor::MoveTo(0, 17))?;
                write!(stdout, "{TEMPLATE_BOTTOM}")?;
                execute!(stdout, terminal::Clear(ClearType::FromCursorDown))?;
                return Ok(());
            },
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Backspace,
                ..
            }) => {
                out.pop();
            },
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char(c),
                ..
            }) => {
                out.push(c);
            },
            _ => (),
        }
    }
}

fn print_info(
    stdout: &mut std::io::Stdout,
    computer: &types::Computer,
    filename: &str,
    output: &str,
) -> Result<(), Error> {
    let prog_len = min(computer.tape.len(), 75);
    let prog_spaces = 75 - prog_len;
    let prog_start;
    let prog_end;
    if computer.tape.len() <= 75 {
        prog_start = 0;
        prog_end = prog_len;
    } else {
        prog_end = max(min(computer.position + 37, computer.tape.len()), 75);
        prog_start = prog_end - 75;
    }
    execute!(stdout, cursor::MoveTo(2, 1))?;
    for (i, char) in computer
        .tape
        .iter()
        .enumerate()
        .take(prog_end)
        .skip(prog_start)
    {
        if i == computer.position {
            execute!(stdout, style::SetAttribute(style::Attribute::Reverse))?;
        }
        write!(stdout, "{}", printable(*char))?;
        if i == computer.position {
            execute!(stdout, style::SetAttribute(style::Attribute::NoReverse))?;
        }
    }
    for _ in 0..=prog_spaces {
        write!(stdout, " ")?;
    }
    execute!(stdout, cursor::MoveTo(10, 3))?;
    #[cfg(feature = "u32")]
    {
        write!(
            stdout,
            "{:2x} {:2x} {:2x} {:2x}",
            computer.cell.0 >> 24 as u8,
            computer.cell.0 >> 16 as u8,
            computer.cell.0 >> 8 as u8,
            computer.cell.0 as u8,
        )?;
    }
    #[cfg(not(feature = "u32"))]
    write!(stdout, "         {:2x}", computer.cell.0)?;
    execute!(stdout, cursor::MoveTo(24, 3))?;
    write!(stdout, "{:>10}", computer.cell.0)?;
    execute!(stdout, cursor::MoveTo(37, 3))?;
    #[cfg(feature = "u32")]
    {
        write!(
            stdout,
            "{}{}{}{}",
            printable(((computer.cell.0 >> 24) & 0xFF) as u8),
            printable(((computer.cell.0 >> 16) & 0xFF) as u8),
            printable(((computer.cell.0 >> 8) & 0xFF) as u8),
            printable((computer.cell.0 & 0xFF) as u8),
        )?;
    }
    #[cfg(not(feature = "u32"))]
    write!(stdout, "   {}", printable(computer.cell.0))?;
    execute!(stdout, cursor::MoveTo(54, 3))?;
    if computer.running {
        write!(stdout, "  Running  ")?;
    } else {
        write!(stdout, "Not Running")?;
    }
    execute!(stdout, cursor::MoveTo(76, 3))?;
    if filename.len() > 25 {
        write!(stdout, "< {}", &filename[filename.len() - 23..])?;
    } else {
        write!(stdout, "{filename:<25}")?;
    }
    print_stack(&computer.primary, 5, stdout)?;
    print_stack(&computer.secondary, 7, stdout)?;
    print_output(output, stdout)?;
    execute!(stdout, cursor::MoveTo(0, 17))?;
    Ok(())
}

fn print_stack(
    stack: &[Wrapping<CellSize>],
    line_no: u16,
    stdout: &mut std::io::Stdout,
) -> Result<(), Error> {
    let (skip, show_more) = if stack.len() > 13 {
        (stack.len() - 13, true)
    } else {
        (0, false)
    };
    for (i, val) in stack
        .iter()
        .skip(skip)
        .map(|i| represent_value(i.0))
        .chain(std::iter::repeat("    ".to_string()))
        .take(13)
        .enumerate()
    {
        #[allow(clippy::cast_possible_truncation)]
        execute!(stdout, cursor::MoveTo((13 + 7 * i) as u16, line_no))?;
        write!(stdout, "{val}")?;
    }
    if show_more {
        execute!(stdout, cursor::MoveTo(13, line_no))?;
        write!(stdout, "<...")?;
    }
    Ok(())
}

fn print_output(output: &str, stdout: &mut std::io::Stdout) -> Result<(), Error> {
    let mut out_buffer = [[' '; 99]; 5];
    let mut y = 0;
    let mut x = 0;
    for char in output.chars() {
        match char {
            '\n' => {
                y += 1;
                x = 0;
            },
            '\r' => {
                x = 0;
            },
            _ => {
                out_buffer[y][x] = char;
                x += 1;
            },
        }
        if x == 99 {
            y += 1;
            x = 0;
        }
        if y == 5 {
            y = 4;
            out_buffer[0].fill(' ');
            out_buffer.rotate_left(1);
        }
    }
    for (y, row) in out_buffer.iter().enumerate() {
        #[allow(clippy::cast_possible_truncation)]
        execute!(stdout, cursor::MoveTo(2, (9 + y) as u16))?;
        for char in row.iter() {
            write!(stdout, "{char}")?;
        }
    }
    Ok(())
}

#[cfg(feature = "u32")]
fn represent_value(c: u32) -> String {
    [
        printable(((c >> 24) & 0xFF) as u8),
        printable(((c >> 16) & 0xFF) as u8),
        printable(((c >> 8) & 0xFF) as u8),
        printable((c & 0xFF) as u8),
    ]
    .iter()
    .collect()
}
#[cfg(not(feature = "u32"))]
fn represent_value(c: u8) -> String {
    format!("{}{:>3}", printable(c), c)
}

const fn printable(c: u8) -> char {
    if c >= 0x20 && c <= 0x7E {
        c as char
    } else {
        '.'
    }
}

fn main() -> Result<(), Error> {
    enable_raw_mode()?;
    execute!(stdout(), cursor::Hide)?;
    let result = run_app();
    execute!(stdout(), cursor::Show)?;
    disable_raw_mode()?;
    result
}
