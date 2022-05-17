/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * Copyright (c) 2022 Ebert Charles Matthee. All rights reserved.
 */

use std::io;

pub fn get_string() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    trim_newline(&mut buffer);
    Ok(buffer)
}

// TODO fix handling of "/" and "\".
// SUGGESTION perhaps check if "/" or "\" after trunc
pub fn get_char() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    buffer.truncate(1);
    Ok(buffer)
}

pub fn get_int() -> io::Result<i32> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let trimmed = buffer.trim();
    match buffer.trim().parse::<i32>() {
        Ok(i) => return Ok(i),
        Err(..) => println!("this was not an integer: {}", trimmed),
    };
    // TODO fix error handling (dont return zero on fail)
    // TODO string.trim() vs fn trim newline
    Ok(0)
}

pub fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}
