/*
    This file is a part of term-string.

    Copyright (C) 2018 Mohammad AlSaleh <CE.Mohammad.AlSaleh at gmail.com>
    https://github.com/rust-alt/term-string

    This Source Code Form is subject to the terms of the Mozilla Public
    License, v. 2.0. If a copy of the MPL was not distributed with this
    file, You can obtain one at <http://mozilla.org/MPL/2.0/>.
*/

#[cfg(test)]
mod tests;
#[macro_use]
mod macros;

pub use term::{terminfo, Terminal};

#[cfg(windows)]
pub use term::WinConsole;

use isatty;

use std::borrow::Borrow;
use std::io::{self, Write};
use std::ops::{Add, AddAssign};

use error::Result;
use style::TermStyle;

#[cfg(not(windows))]
pub trait TermWrite: Write {}

// Send required by WinConsole
#[cfg(windows)]
pub trait TermWrite: Write + Send {}

#[cfg(not(windows))]
impl<W> TermWrite for W where W: Write {}
#[cfg(windows)]
impl<W> TermWrite for W where W: Write + Send {}

#[derive(Clone, Default, PartialEq, Debug)]
struct TermStringElement {
    style: TermStyle,
    text: String,
}

impl TermStringElement {
    fn new(style: TermStyle, text: &str) -> Self {
        Self {
            style,
            text: String::from(text),
        }
    }
}

impl TermStringElement {
    fn try_write_styled<W, TERM>(&self, out: &mut TERM) -> Result<()>
    where
        W: TermWrite,
        TERM: Terminal<Output = W>,
    {
        // It's important to reset so text with empty style does not inherit attrs
        out.reset()?;

        for attr in self.style.attrs.iter().filter_map(|&attr| attr) {
            out.attr(attr)?;
        }

        write!(out, "{}", self.text)?;

        // Ignore the error here to avoid double writes
        let _ = out.reset();

        Ok(())
    }

    fn write_plain<W: TermWrite>(&self, out_plain: &mut W) {
        write!(out_plain, "{}", &self.text).expect("should never happen");
    }

    fn write_styled<W, TERM>(&self, out_term: &mut TERM, out_plain: &mut W)
    where
        W: TermWrite,
        TERM: Terminal<Output = W>,
    {
        if self.try_write_styled(out_term).is_err() {
            self.write_plain(out_plain);
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct TermString {
    elements: Vec<TermStringElement>,
}

// Essentials
impl TermString {
    pub fn new(style: TermStyle, text: &str) -> Self {
        let mut elements = Vec::with_capacity(128);
        elements.push(TermStringElement::new(style, text));
        Self { elements }
    }

    pub fn len(&self) -> usize {
        self.elements.iter().fold(0, |acc, e| acc + e.text.len())
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn as_string(&self) -> String {
        self.elements
            .iter()
            .fold(String::with_capacity(1024), |acc, e| acc + &e.text)
    }

    pub fn append_str<S: Borrow<str>>(&mut self, text: S) {
        if self.elements.last().is_none() {
            self.append(text);
        } else if let Some(last) = self.elements.last_mut() {
            last.text += text.borrow();
        }
    }

    pub fn with_appended_str<S: Borrow<str>>(mut self, text: S) -> Self {
        self.append_str(text);
        self
    }

    pub fn append<IS: Into<Self>>(&mut self, other: IS) {
        let mut other = other.into();
        self.elements.retain(|e| *e != TermStringElement::default());
        other
            .elements
            .retain(|e| *e != TermStringElement::default());

        let mut other_elements_iter = other.elements.into_iter();

        if self.elements.len() > 0 {
            while let Some(next) = other_elements_iter.next() {
                if next.style == self.elements.last().expect("impossible").style {
                    self.append_str(&*next.text);
                } else {
                    self.elements.push(next);
                    break;
                }
            }
            self.elements.extend(other_elements_iter);
        } else {
            self.elements.extend(other_elements_iter);
        }
    }

    pub fn with_appended<IS: Into<Self>>(mut self, other: IS) -> Self {
        self.append(other);
        self
    }
}

// Style
impl TermString {
    pub fn set_style<IT: Into<TermStyle>>(&mut self, style: IT) {
        let style = style.into();
        self.elements.iter_mut().for_each(|f| f.style = style);
    }

    pub fn with_set_style<IT: Into<TermStyle>>(mut self, style: IT) -> Self {
        self.set_style(style);
        self
    }

    pub fn reset_style(&mut self) {
        self.elements.iter_mut().for_each(|f| f.style.reset());
    }

    pub fn with_reset_style(mut self) -> Self {
        self.reset_style();
        self
    }

    pub fn or_style<IT: Into<TermStyle>>(&mut self, style: IT) {
        let style = style.into();
        self.elements
            .iter_mut()
            .for_each(|f| f.style.or_style(style));
    }

    pub fn with_ored_style<IT: Into<TermStyle>>(mut self, style: IT) -> Self {
        self.or_style(style);
        self
    }

    pub fn add_style<IT: Into<TermStyle>>(&mut self, style: IT) {
        let style = style.into();
        self.elements
            .iter_mut()
            .for_each(|f| f.style.add_style(style));
    }

    pub fn with_style<IT: Into<TermStyle>>(mut self, style: IT) -> Self {
        self.add_style(style);
        self
    }
}

// write/print

gen_idents!(print, eprint, stdout, stderr);

impl TermString {
    pub fn write_plain<F, W>(&self, out: &F)
    where
        W: TermWrite,
        F: Fn() -> W,
    {
        let mut out_plain = out();
        for e in &self.elements {
            e.write_plain(&mut out_plain);
        }
    }

    #[cfg(not(windows))]
    pub fn write_styled<F, W>(&self, out: &F)
    where
        W: TermWrite,
        F: Fn() -> W,
    {
        let mut out_plain = out();

        match terminfo::TerminfoTerminal::new(out()) {
            Some(mut out_term) => {
                for e in &self.elements {
                    e.write_styled(&mut out_term, &mut out_plain);
                }
            },
            None => self.write_plain(out),
        }
    }

    #[cfg(windows)]
    pub fn write_styled<F, W>(&self, out: &F)
    where
        W: TermWrite,
        F: Fn() -> W,
    {
        let mut out_plain = out();

        match (
            terminfo::TerminfoTerminal::new(out()),
            WinConsole::new(out()),
        ) {
            (Some(mut out_term), _) => {
                for e in &self.elements {
                    e.write_styled(&mut out_term, &mut out_plain);
                }
            },
            (_, Ok(mut out_term)) => {
                for e in &self.elements {
                    e.write_styled(&mut out_term, &mut out_plain);
                }
            },
            _ => self.write_plain(out),
        }
    }

    gen_print_fns!(stdout, print);
    gen_print_fns!(stderr, eprint);
}

impl<B: Borrow<str>> From<B> for TermString {
    fn from(s: B) -> Self {
        Self::new(TermStyle::default(), s.borrow())
    }
}

impl<B: Borrow<str>> Add<B> for TermString {
    type Output = Self;
    fn add(self, text: B) -> Self {
        self.with_appended_str(text)
    }
}

impl<B: Borrow<str>> AddAssign<B> for TermString {
    fn add_assign(&mut self, text: B) {
        self.append_str(text);
    }
}

impl Add for TermString {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        self.with_appended(other)
    }
}

impl AddAssign for TermString {
    fn add_assign(&mut self, other: Self) {
        self.append(other);
    }
}
