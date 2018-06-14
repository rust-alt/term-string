/*
    This file is a part of term-string.

    Copyright (C) 2018 Mohammad AlSaleh <CE.Mohammad.AlSaleh at gmail.com>
    https://github.com/rust-alt/term-string

    This Source Code Form is subject to the terms of the Mozilla Public
    License, v. 2.0. If a copy of the MPL was not distributed with this
    file, You can obtain one at <http://mozilla.org/MPL/2.0/>.
*/

use term;

use std::{fmt, result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    StdIO(::std::io::Error),
    Term(term::Error),
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::StdIO(ref e) => write!(f, "IO Error: {}", e),
            Error::Term(ref e) => write!(f, "Term Error: {}", e),
            Error::Other(ref e) => write!(f, "Error: {}", e),
        }
    }
}

impl From<::std::io::Error> for Error {
    fn from(e: ::std::io::Error) -> Self {
        Error::StdIO(e)
    }
}

impl From<term::Error> for Error {
    fn from(e: term::Error) -> Self {
        Error::Term(e)
    }
}

impl<'a> From<&'a str> for Error {
    fn from(e: &str) -> Self {
        Error::Other(e.into())
    }
}
