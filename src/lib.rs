/*
    This file is a part of term-string.

    Copyright (C) 2018 Mohammad AlSaleh <CE.Mohammad.AlSaleh at gmail.com>
    https://github.com/rust-alt/term-string

    This Source Code Form is subject to the terms of the Mozilla Public
    License, v. 2.0. If a copy of the MPL was not distributed with this
    file, You can obtain one at <http://mozilla.org/MPL/2.0/>.
*/

//! Provide a string and style types for terminal formatted output.
//!
//! # Examples
//!
//! ``` rust
//! // color is a re-exported module from term
//! use term_string::color::{GREEN, RED};
//!
//! use term_string::{TermString, TermStyle};
//!
//! // Create term styles
//! let style1 = TermStyle::bold() + TermStyle::fg(GREEN) + TermStyle::bg(RED);
//! let style2 = TermStyle::bold();
//!
//! // Create term string from a style and a string value
//! let mut ts = TermString::new(style1, "Style");
//!
//! // Append string values with different styles.
//!
//! // ts += TermString::new(style1, "1");
//! ts+= "1";
//!
//! // ts += TermString::new(TermStyle::default(), " : ");
//! ts += TermString::from(" : ");
//!
//! ts += TermString::new(style2, "This is text with style 2");
//!
//! ts.println();
//! ```
//!
//! Check [`TermStyle`] and [`TermString`] for detailed documentation of each type.

#![recursion_limit = "1024"]

#[macro_use]
extern crate mashup;

pub extern crate isatty;
pub extern crate term;

#[macro_use]
mod common_macros;

mod error;
mod string;
mod style;

#[doc(inline)]
pub use style::{color, Attr, TermStyle};

#[doc(inline)]
pub use string::{TermString, TermWrite};
