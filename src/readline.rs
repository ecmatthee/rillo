/*
    This Source Code Form is subject to the terms of the Mozilla Public
    License, v. 2.0. If a copy of the MPL was not distributed with this
    file, You can obtain one at http://mozilla.org/MPL/2.0/.

    Copyright (c) 2022 Ebert Charles Matthee. All rights reserved.
*/

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    style, ExecutableCommand, QueueableCommand,
};
use std::io::{self, stdout, Stdout, Write};

//TODO Line Wrapping, Style, Multiline editing

pub struct ReadLine {
    pub echo: bool,
    pub prompt: String,
    pub ignored: Vec<KeyCode>,
    pub dimensions: (u16, u16),
}

impl ReadLine {
    pub fn read_loop(&self) -> io::Result<String> {
        let mut stdout = stdout();
        let mut buf = String::new();
        stdout.execute(style::Print(&self.prompt))?;
        stdout.write(b"\x08");
        stdout.execute(cursor::SetCursorShape(cursor::CursorShape::Block));

        let origin = cursor::position()?;

        while let Event::Key(KeyEvent { code, modifiers }) = event::read()? {
            if !(self.ignored.iter().any(|&i| i == code)) {
                match (code, modifiers) {
                    (KeyCode::Enter, KeyModifiers::NONE) => {
                        break;
                    }
                    (KeyCode::Right, KeyModifiers::NONE) => {
                        let bchar = cursor::position().unwrap().0 - origin.0;
                        let b = buf.chars().count() as u16;
                        if bchar < b {
                            stdout.queue(cursor::MoveRight(1))?;
                            stdout.flush()?;
                        }
                    }
                    (KeyCode::Left, KeyModifiers::NONE) => {
                        let bchar = cursor::position()?;
                        if bchar.0 > origin.0 {
                            stdout.queue(cursor::MoveLeft(1))?;
                            stdout.flush()?;
                        }
                    }
                    (KeyCode::Backspace, KeyModifiers::NONE) => {
                        let mut bchar = cursor::position()?;

                        if bchar.0 > origin.0 {
                            stdout.queue(cursor::MoveLeft(1))?;
                            bchar.0 -= 1;

                            clear_string(&mut stdout, &mut buf, &origin, &bchar)?;
                            buf.remove(bchar.0 as usize - origin.0 as usize);
                            reprint_string(self.echo, &mut stdout, &buf, &origin)?;
                            stdout.queue(cursor::MoveTo(bchar.0, bchar.1))?;
                            stdout.flush()?;
                        }
                    }
                    (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                        break;
                    }
                    (KeyCode::Char(c), KeyModifiers::NONE) => {
                        let bchar = cursor::position()?;
                        let x = bchar.0 - origin.0;
                        buf.insert(x.into(), c);
                        reprint_string(self.echo, &mut stdout, &buf, &origin)?;
                        stdout.queue(cursor::MoveTo(bchar.0 + 1, bchar.1))?;
                        stdout.flush()?;
                    }
                    (KeyCode::Char(c), KeyModifiers::SHIFT) => {
                        let bchar = cursor::position()?;
                        let x = bchar.0 - origin.0;
                        buf.insert(x.into(), c);
                        reprint_string(self.echo, &mut stdout, &buf, &origin)?;
                        stdout.queue(cursor::MoveTo(bchar.0 + 1, bchar.1))?;
                        stdout.flush()?;
                    }
                    _ => {}
                }
            }
        }

        Ok(buf)
    }
}

fn clear_string(
    stdout: &mut Stdout,
    buf: &mut String,
    origin: &(u16, u16),
    clear_start: &(u16, u16),
) -> io::Result<()> {
    let string_end = buf.chars().count() as u16 + origin.0;

    for x in clear_start.0..string_end {
        stdout.queue(cursor::MoveTo(x, clear_start.1))?;
        stdout.queue(style::Print(" "))?;
    }

    Ok(())
}

fn reprint_string(
    echo: bool,
    stdout: &mut Stdout,
    buf: &String,
    origin: &(u16, u16),
) -> io::Result<()> {
    if echo {
        stdout.queue(cursor::MoveTo(origin.0, origin.1))?;
        stdout.queue(style::Print(&buf))?;
    }
    Ok(())
}
