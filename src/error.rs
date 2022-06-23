/*
    This Source Code Form is subject to the terms of the Mozilla Public
    License, v. 2.0. If a copy of the MPL was not distributed with this
    file, You can obtain one at http://mozilla.org/MPL/2.0/.

    Copyright (c) 2022 Ebert Charles Matthee. All rights reserved.
*/

use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReadlineError {
    #[error("io error")]
    IOError(#[from] io::Error),
    #[error("CTRL-C")]
    Interrupted,
    #[error("CTRL-D")]
    EoF,
    #[error("unkown readline error")]
    Unknown,
}
