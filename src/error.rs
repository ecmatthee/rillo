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
