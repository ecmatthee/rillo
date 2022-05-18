/*
    This Source Code Form is subject to the terms of the Mozilla Public
    License, v. 2.0. If a copy of the MPL was not distributed with this
    file, You can obtain one at http://mozilla.org/MPL/2.0/.

    Copyright (c) 2022 Ebert Charles Matthee. All rights reserved.
*/

use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal, QueueableCommand,
};
use std::io;
use std::io::stdout;

pub struct Rectangle {
    // TODO Color as field
    pub origin: (u16, u16),
    pub size_col: u16,
    pub size_row: u16,
    pub char_corner_top_left: char,
    pub char_corner_top_right: char,
    pub char_corner_bottom_left: char,
    pub char_corner_bottom_right: char,
    pub char_horizontal_top: char,
    pub char_horizontal_bottom: char,
    pub char_verticle_left: char,
    pub char_verticle_right: char,
    pub char_fill: char,
}

impl Rectangle {
    pub fn draw(&self) -> io::Result<()> {
        let mut stdout = stdout();

        let col = self.size_col - 1;
        let row = self.size_row - 1;

        let term_max = match terminal::size() {
            Ok(i) => i,
            Err(..) => (1, 1),
        };

        // Check if drawing will on current terminal screen size
        if (self.origin.0 + col > term_max.0) || (self.origin.1 + row > term_max.1) {
            // TODO Error handling
            return Ok(());
        };

        // Sides
        for x in 1..col {
            stdout.queue(cursor::MoveTo(self.origin.0 + x, self.origin.1))?;
            stdout.queue(style::PrintStyledContent(
                self.char_horizontal_top.clone().magenta(),
            ))?;

            stdout.queue(cursor::MoveTo(self.origin.0 + x, self.origin.1 + row))?;
            stdout.queue(style::PrintStyledContent(
                self.char_horizontal_bottom.clone().magenta(),
            ))?;
        }

        for y in 1..row {
            stdout.queue(cursor::MoveTo(self.origin.0, self.origin.1 + y))?;
            stdout.queue(style::PrintStyledContent(
                self.char_verticle_left.clone().magenta(),
            ))?;

            stdout.queue(cursor::MoveTo(self.origin.0 + col, self.origin.1 + y))?;
            stdout.queue(style::PrintStyledContent(
                self.char_verticle_right.clone().magenta(),
            ))?;
        }

        // Corners
        stdout.queue(cursor::MoveTo(self.origin.0, self.origin.1))?;
        stdout.queue(style::PrintStyledContent(
            self.char_corner_top_left.clone().magenta(),
        ))?;

        stdout.queue(cursor::MoveTo(self.origin.0 + col, self.origin.1))?;
        stdout.queue(style::PrintStyledContent(
            self.char_corner_top_right.clone().magenta(),
        ))?;

        stdout.queue(cursor::MoveTo(self.origin.0, self.origin.1 + row))?;
        stdout.queue(style::PrintStyledContent(
            self.char_corner_bottom_left.clone().magenta(),
        ))?;

        stdout.queue(cursor::MoveTo(self.origin.0 + col, self.origin.1 + row))?;
        stdout.queue(style::PrintStyledContent(
            self.char_corner_bottom_right.clone().magenta(),
        ))?;

        Ok(())
    }

    pub fn fill(&self) -> io::Result<()> {
        todo!();
    }

    pub fn erase(&self) -> io::Result<()> {
        todo!();
    }

    pub fn erase_fill(&self) -> io::Result<()> {
        todo!();
    }
}

pub struct Line {
    start_point: (u16, u16),
    end_point: (u16, u16),
    char_line: char,
}

impl Line {
    // add code here
    pub fn draw(&self) -> io::Result<()> {
        todo!();
    }

    pub fn erase(&self) -> io::Result<()> {
        todo!();
    }
}
