/*
    This Source Code Form is subject to the terms of the Mozilla Public
    License, v. 2.0. If a copy of the MPL was not distributed with this
    file, You can obtain one at http://mozilla.org/MPL/2.0/.

    Copyright (c) 2022 Ebert Charles Matthee. All rights reserved.
*/
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    style,
    terminal::{disable_raw_mode, enable_raw_mode, is_raw_mode_enabled},
    ExecutableCommand, QueueableCommand,
};
use std::io::{stdout, Result, Write};

use crate::terminal::term_size;

pub struct Readline {
    pub echo: bool,
    pub prompt: String,
    pub origin: (u16, u16),
    pub line_length: u16,
    pub ignored_keys: Vec<(KeyCode, KeyModifiers)>,
}

impl Default for Readline {
    fn default() -> Self {
        let ignored_list: Vec<(KeyCode, KeyModifiers)> = Vec::new();
        Readline {
            echo: true,
            prompt: ">> ".to_string(),
            origin: (0, 0),
            line_length: 0,
            ignored_keys: ignored_list,
        }
    }
}

impl Readline {
    pub fn readline(&self) -> Result<String> {
        let mut raw_manual_enable = false;
        if !is_raw_mode_enabled()? {
            enable_raw_mode()?;
            raw_manual_enable = true;
        }

        let mut stdout = stdout();
        let mut line = String::new();

        // stdout.queue(cursor::MoveTo(self.origin.0, self.origin.1))?;
        stdout.queue(style::Print(&self.prompt))?;
        stdout.flush()?;

        let string_origin = cursor::position()?;

        while let Event::Key(KeyEvent { code, modifiers }) = event::read()? {
            let position = cursor::position()?;
            let string_position = position.0 - string_origin.0;
            if !(self.ignored_keys.iter().any(|&i| i == (code, modifiers))) {
                // println!("{:?}", modifiers);
                match (code, modifiers) {
                    (KeyCode::Enter, KeyModifiers::NONE) => {
                        if raw_manual_enable {
                            disable_raw_mode()?;
                        };
                        break;
                    }
                    (KeyCode::Backspace, KeyModifiers::NONE) => {
                        if position > string_origin {
                            line.remove(string_position as usize - 1);
                            reprint_line(&line, &string_origin, &self.line_length)?;
                            stdout.queue(cursor::MoveTo(position.0 - 1, position.1))?;
                            stdout.flush()?;
                        }
                    }
                    (KeyCode::Left, KeyModifiers::NONE) => {
                        if position.0 > string_origin.0 {
                            stdout.execute(cursor::MoveLeft(1))?;
                        }
                    }
                    (KeyCode::Right, KeyModifiers::NONE) => {
                        let string_length = line.chars().count() as u16;
                        if string_position < string_length {
                            stdout.execute(cursor::MoveRight(1))?;
                        }
                    }
                    (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                        if raw_manual_enable {
                            disable_raw_mode()?;
                        };
                        println!("CTRL-C");
                    }
                    (KeyCode::Char('c'), _) => {
                        if modifiers == (KeyModifiers::CONTROL | KeyModifiers::ALT) {
                            if raw_manual_enable {
                                disable_raw_mode()?;
                            };
                            println!("Wo");
                            break;
                        };
                    }
                    (KeyCode::Char(char), KeyModifiers::NONE) => {
                        line.insert(string_position.into(), char);
                        reprint_line(&line, &string_origin, &self.line_length)?;
                        stdout.queue(cursor::MoveTo(position.0 + 1, position.1))?;
                        stdout.flush()?;
                    }
                    (KeyCode::Char(char), KeyModifiers::SHIFT) => {
                        line.insert(string_position.into(), char);
                        reprint_line(&line, &string_origin, &self.line_length)?;
                        stdout.queue(cursor::MoveTo(position.0 + 1, position.1))?;
                        stdout.flush()?;
                    }
                    _ => {}
                }
            };
        }

        Ok(line)
    }
}

fn clear_line(string_origin: &(u16, u16), line_length: &u16) -> Result<()> {
    let mut stdout = stdout();
    let mut clear_length = *line_length;
    if clear_length == 0 {
        clear_length = term_size().0;
    }

    for x in string_origin.0..clear_length {
        stdout.queue(cursor::MoveTo(x, string_origin.1))?;
        stdout.queue(style::Print(" "))?;
    }
    Ok(())
}

fn reprint_line(line: &String, string_origin: &(u16, u16), line_length: &u16) -> Result<()> {
    let mut stdout = stdout();
    clear_line(&string_origin, line_length)?;
    stdout.queue(cursor::MoveTo(string_origin.0, string_origin.1))?;
    stdout.queue(style::Print(line))?;
    Ok(())
}
