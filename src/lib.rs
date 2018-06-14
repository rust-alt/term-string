/*
    This file is a part of term-string.

    Copyright (C) 2018 Mohammad AlSaleh <CE.Mohammad.AlSaleh at gmail.com>
    https://github.com/rust-alt/term-string

    This Source Code Form is subject to the terms of the Mozilla Public
    License, v. 2.0. If a copy of the MPL was not distributed with this
    file, You can obtain one at <http://mozilla.org/MPL/2.0/>.
*/

#![recursion_limit="2048"]

#[macro_use]
extern crate mashup;

pub extern crate term;
pub extern crate isatty;

mod error;
mod style;
mod string;

#[doc(inline)]
pub use style::{TermStyle, Attr, color};

#[doc(inline)]
pub use string::TermString;
