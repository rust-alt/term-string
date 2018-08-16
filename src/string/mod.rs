/*
    This file is a part of term-string.

    Copyright (C) 2018 Mohammad AlSaleh <CE.Mohammad.AlSaleh at gmail.com>
    https://github.com/rust-alt/term-string

    This Source Code Form is subject to the terms of the Mozilla Public
    License, v. 2.0. If a copy of the MPL was not distributed with this
    file, You can obtain one at <http://mozilla.org/MPL/2.0/>.
*/

// TODO: explore indexing and PartialEq

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

    fn write_plain<W>(&self, out_plain: &mut W)
    where
        W: TermWrite,
    {
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
/// Basic methods for constructing and modifying [`TermString`]s,
impl TermString {
    /// Create a [`TermString`] variable from a [`TermStyle`] and a string value.
    ///
    /// # Examples
    ///
    /// ``` rust
    /// # use term_string::{TermString, TermStyle};
    /// let bold = TermStyle::bold();
    /// let ts = TermString::new(bold, "some bold text");
    /// ts.println();
    /// ```
    pub fn new<S>(style: TermStyle, text: S) -> Self
    where
        S: Borrow<str>,
    {
        let mut elements = Vec::with_capacity(128);
        elements.push(TermStringElement::new(style, text.borrow()));
        Self { elements }
    }

    /// Return the length of the un-styled string contained in [`TermString`].
    ///
    /// # Examples
    ///
    /// ``` rust
    /// # use term_string::{TermString, TermStyle};
    /// let bold = TermStyle::bold();
    /// let underline = TermStyle::underline(true);
    /// let mut ts = TermString::new(bold, "some bold text ");
    /// ts += TermString::new(underline, "and some underlined text.");
    /// assert_eq!(ts.len(), 40);
    /// ```
    pub fn len(&self) -> usize {
        self.elements.iter().fold(0, |acc, e| acc + e.text.len())
    }

    /// Return true if the un-styled string contained in [`TermString`]
    /// is empty.
    ///
    /// # Examples
    ///
    /// ``` rust
    /// # use term_string::{TermString, TermStyle};
    /// let bold = TermStyle::bold();
    /// let mut ts = TermString::new(bold, "");
    /// assert!(ts.is_empty());
    /// ts += "this is bold."
    /// ```
    // Note: empty does not imply the struct's internal vector is also empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Return the un-styled string contained in [`TermString`].
    ///
    /// # Examples
    ///
    /// ``` rust
    /// # use term_string::{TermString, TermStyle};
    /// let bold = TermStyle::bold();
    /// let underline = TermStyle::underline(true);
    /// let mut ts = TermString::new(bold, "some bold text ");
    /// ts += TermString::new(underline, "and some underlined text.");
    /// let s = "some bold text and some underlined text.";
    /// assert_eq!(ts.as_string(), s);
    /// ```
    pub fn as_string(&self) -> String {
        self.elements
            .iter()
            .fold(String::with_capacity(1024), |acc, e| acc + &e.text)
    }

    /// Append a string value to a [`TermString`], inheriting the previous style.
    ///
    /// # Examples
    ///
    /// ``` rust
    /// # use term_string::{TermString, TermStyle};
    /// let bold = TermStyle::bold();
    /// let mut ts = TermString::new(bold, "some bold text ");
    /// ts.append_str("and other bold text");
    /// ts.println();
    /// ```
    ///
    /// Note that the line:
    ///
    /// ``` rust
    /// # use term_string::{TermString, TermStyle};
    /// # let bold = TermStyle::bold();
    /// # let mut ts = TermString::new(bold, "some bold text ");
    /// ts.append_str("and other bold text");
    /// ```
    ///
    ///  is equivalent to:
    ///
    ///  ``` rust
    /// # use term_string::{TermString, TermStyle};
    /// # let bold = TermStyle::bold();
    /// # let mut ts = TermString::new(bold, "some bold text ");
    /// ts += "and other bold text";
    ///  ```
    pub fn append_str<S>(&mut self, text: S)
    where
        S: Borrow<str>,
    {
        if self.elements.last().is_none() {
            self.append_term_str(text);
        } else if let Some(last) = self.elements.last_mut() {
            last.text += text.borrow();
        }
    }

    /// Append a [`TermString`] to a [`TermString`].
    ///
    /// # Examples
    ///
    /// ``` rust
    /// # use term_string::{TermString, TermStyle};
    /// let bold = TermStyle::bold();
    /// let underline = TermStyle::underline(true);
    /// let mut ts = TermString::new(bold, "some bold text ");
    /// let ts2 = TermString::new(underline, "and some underlined text.");
    /// ts.append_term_str(ts2);
    /// ts.println();
    /// ```
    ///
    /// Note that the line:
    ///
    /// ``` rust
    /// # use term_string::{TermString, TermStyle};
    /// # let bold = TermStyle::bold();
    /// # let underline = TermStyle::underline(true);
    /// # let mut ts = TermString::new(bold, "some bold text ");
    /// # let ts2 = TermString::new(underline, "and some underlined text.");
    /// ts.append_term_str(ts2);
    /// ```
    ///
    ///  is equivalent to:
    ///
    /// ``` rust
    /// # use term_string::{TermString, TermStyle};
    /// # let bold = TermStyle::bold();
    /// # let underline = TermStyle::underline(true);
    /// # let mut ts = TermString::new(bold, "some bold text ");
    /// # let ts2 = TermString::new(underline, "and some underlined text.");
    /// ts += ts2;
    /// ```
    ///
    /// Also note that the method's argument type is `Into<Self>`, and
    /// `From<S> for TermString where S: Borrow<str>` is implemented.
    ///
    /// So, this works:
    ///
    /// ``` rust
    /// # use term_string::{TermString, TermStyle};
    /// let bold = TermStyle::bold();
    /// let mut ts = TermString::new(bold, "some bold text ");
    /// ts.append_term_str("and some un-styled text.");
    /// ts.println();
    /// ```
    /// Note that the method argument in the example above is converted
    /// into a [`TermString`] with a `Default` style first before appending.
    /// Contrast this with the behavior of [`append_str()`], where the appended
    /// value inherits the previous style.
    ///
    /// [`append_str()`]: TermString::append_str()
    ///
    /// So, the line:
    ///
    /// ``` rust
    /// # use term_string::{TermString, TermStyle};
    /// # let bold = TermStyle::bold();
    /// # let mut ts = TermString::new(bold, "some bold text ");
    /// ts.append_term_str("and some un-styled text.");
    ///```
    ///
    ///  is equivalent to:
    ///
    /// ``` rust
    /// # use term_string::{TermString, TermStyle};
    /// # let bold = TermStyle::bold();
    /// # let mut ts = TermString::new(bold, "some bold text ");
    /// ts += TermString::from("and some un-styled text.");
    /// ```
    ///
    /// which in turn is equivalent to:
    ///
    /// ``` rust
    /// # use term_string::{TermString, TermStyle};
    /// # let bold = TermStyle::bold();
    /// # let mut ts = TermString::new(bold, "some bold text ");
    /// ts += TermString::new(TermStyle::default(), "and some un-styled text.");
    /// ```
    pub fn append_term_str<IS>(&mut self, other: IS)
    where
        IS: Into<Self>,
    {
        let mut other = other.into();
        self.elements.retain(|e| *e != TermStringElement::default());
        other
            .elements
            .retain(|e| *e != TermStringElement::default());

        let mut other_elements_iter = other.elements.into_iter();

        if !self.elements.is_empty() {
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

    /// This is effectively an alias to [`append_term_str()`]
    ///
    /// [`append_term_str()`]: TermString::append_term_str
    pub fn append<IS>(&mut self, other: IS)
    where
        IS: Into<Self>,
    {
        self.append_term_str(other);
    }

    chaining_fn!(TermString, append_str,
               pub fn with_appended_str<S>(mut self, text: S) -> Self
               where
                   S: Borrow<str>,
               {
                   self.append_str(text);
                   self
               }
    );

    chaining_fn!(TermString, append_term_str,
                  pub fn with_appended_term_str<IS>(mut self, other: IS) -> Self
                  where
                      IS: Into<Self>,
                  {
                      self.append_term_str(other);
                      self
                  }
    );

    chaining_fn!(TermString, append,
                  pub fn with_appended<IS>(mut self, other: IS) -> Self
                  where
                      IS: Into<Self>,
                  {
                      self.append(other);
                      self
                  }
    );
}

// Style
/// Method for modifying the style of all internal elements of a [`TermString`].
///
/// A corresponding method from [`TermStyle`] is used on each internal element
/// of the [`TermString`].
///
/// Remember that [`TermStyle`] is a `Copy` type.
impl TermString {
    /// Set the styles of all internal elements of the [`TermString`] to this style.
    ///
    /// # Examples
    ///
    /// ``` rust
    /// use term_string::{TermString, TermStyle, color};
    ///
    /// let fg_bg = TermStyle::bg(color::RED) + TermStyle::fg(color::WHITE);
    /// let underline = TermStyle::underline(true);
    ///
    /// let mut ts = TermString::new(fg_bg, "fg bg");
    /// ts += TermString::new(underline, " underline");
    ///
    /// ts.set_style(TermStyle::bold());
    ///
    /// // This will print "fg bg underline" in bold and without
    /// // foreground or background colors or underline.
    /// ts.println();
    /// ```
    pub fn set_style<IT>(&mut self, style: IT)
    where
        IT: Into<TermStyle>,
    {
        let style = style.into();
        self.elements.iter_mut().for_each(|f| f.style = style);
    }

    chaining_fn!(TermString, set_style,
                  pub fn with_set_style<IT>(mut self, style: IT) -> Self
                  where
                      IT: Into<TermStyle>,
                  {
                      self.set_style(style);
                      self
                  }
    );

    /// Reset the styles of all internal elements of the [`TermString`].
    ///
    /// # Examples
    ///
    /// ``` rust
    /// use term_string::{TermString, TermStyle, color};
    ///
    /// let fg_bg = TermStyle::bg(color::RED) + TermStyle::fg(color::WHITE);
    /// let underline = TermStyle::underline(true);
    ///
    /// let mut ts = TermString::new(fg_bg, "fg bg");
    /// ts += TermString::new(underline, " underline");
    ///
    /// ts.reset_style();
    ///
    /// // This will print "fg bg underline" without any styling
    /// ts.println();
    /// ```
    pub fn reset_style(&mut self) {
        self.elements.iter_mut().for_each(|f| f.style.reset());
    }

    chaining_fn!(TermString, reset_style,
                 pub fn with_reset_style(mut self) -> Self {
                     self.reset_style();
                     self
                 }
    );

    /// Calls [`TermStyle::or_style()`] on each internal element of the [`TermString`].
    ///
    /// # Examples
    ///
    /// ``` rust
    /// use term_string::{TermString, TermStyle, color};
    ///
    /// let fg_bg = TermStyle::bg(color::RED) + TermStyle::fg(color::WHITE);
    /// let underline = TermStyle::underline(true);
    ///
    /// let mut ts = TermString::new(fg_bg, "fg bg");
    /// ts += TermString::new(underline, " underline");
    ///
    /// ts.or_style(TermStyle::bg(color::BLUE));
    ///
    /// // This will print "fg bg" with red background and white foreground,
    /// // then " underline" with underline and blue background.
    /// ts.println();
    /// ```
    pub fn or_style<IT>(&mut self, style: IT)
    where
        IT: Into<TermStyle>,
    {
        let style = style.into();
        self.elements
            .iter_mut()
            .for_each(|f| f.style.or_style(style));
    }

    chaining_fn!(TermString, or_style,
                 pub fn with_ored_style<IT>(mut self, style: IT) -> Self
                 where
                     IT: Into<TermStyle>,
                 {
                     self.or_style(style);
                     self
                 }
    );

    /// Calls [`TermStyle::add_style()`] on each internal element of the [`TermString`].
    ///
    /// # Examples
    ///
    /// ``` rust
    /// use term_string::{TermString, TermStyle, color};
    ///
    /// let fg_bg = TermStyle::bg(color::RED) + TermStyle::fg(color::WHITE);
    /// let underline = TermStyle::underline(true);
    ///
    /// let mut ts = TermString::new(fg_bg, "fg bg");
    /// ts += TermString::new(underline, " underline");
    ///
    /// ts.add_style(TermStyle::bg(color::BLUE));
    ///
    /// // This will print "fg bg" with blue background and white foreground,
    /// // then " underline" with underline and blue background.
    /// ts.println();
    /// ```
    pub fn add_style<IT>(&mut self, style: IT)
    where
        IT: Into<TermStyle>,
    {
        let style = style.into();
        self.elements
            .iter_mut()
            .for_each(|f| f.style.add_style(style));
    }

    chaining_fn!(TermString, add_style,
                 pub fn with_style<IT>(mut self, style: IT) -> Self
                 where
                     IT: Into<TermStyle>,
                 {
                     self.add_style(style);
                     self
                 }
    );
}

// write/print

gen_idents!(print, eprint, stdout, stderr);

/// IO write/print methods
///
/// # Note
/// `TermWrite` is bound to `Write + Send` on Windows, and only `Write`
/// on other platforms.
impl TermString {
    /// Write [`TermString`] to `out` without styling.
    ///
    /// # Examples
    /// ``` rust
    /// # use term_string::{TermString, TermStyle};
    /// let bold = TermStyle::bold();
    /// let ts = TermString::new(bold, "some bold text");
    ///
    /// // This will write "some bold text" to stdout without
    /// // any formatting, so not really bold.
    /// ts.write_plain(&|| std::io::stdout());
    /// ```
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
    fn _write_styled<F, W>(&self, out: &F)
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
    fn _write_styled<F, W>(&self, out: &F)
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

    /// Write [`TermString`] to `out` with styling.
    ///
    /// # Note
    ///
    /// `out` doesn't have to be an actual tty.
    ///
    /// Check out [`print()`], [`println()`], [`eprint()`], and [`eprintln()`]
    /// below, where `out` is checked before styled output is written to it.
    ///
    ///
    /// [`print()`]: TermString::print
    /// [`println()`]: TermString::println
    /// [`eprint()`]: TermString::eprint
    /// [`eprintln()`]: TermString::eprintln
    ///
    /// # Examples
    /// ``` rust
    /// # use term_string::{TermString, TermStyle};
    /// let bold = TermStyle::bold();
    /// let ts = TermString::new(bold, "some bold text");
    ///
    /// // This will write "some bold text" to stdout with formatting,
    /// // even if stdout is not a tty
    /// ts.write_styled(&|| std::io::stdout());
    /// ```
    pub fn write_styled<F, W>(&self, out: &F)
    where
        W: TermWrite,
        F: Fn() -> W,
    {
        self._write_styled(out)
    }

    gen_print_fns!(stdout, print);
    gen_print_fns!(stderr, eprint);
}

impl<S> From<S> for TermString
where
    S: Borrow<str>,
{
    fn from(s: S) -> Self {
        Self::new(TermStyle::default(), s.borrow())
    }
}

impl<S> Add<S> for TermString
where
    S: Borrow<str>,
{
    type Output = Self;
    fn add(self, text: S) -> Self {
        self.with_appended_str(text)
    }
}

impl<S> AddAssign<S> for TermString
where
    S: Borrow<str>,
{
    fn add_assign(&mut self, text: S) {
        self.append_str(text);
    }
}

impl Add for TermString {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        self.with_appended_term_str(other)
    }
}

impl AddAssign for TermString {
    fn add_assign(&mut self, other: Self) {
        self.append_term_str(other);
    }
}
