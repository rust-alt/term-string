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

pub use term::{Attr, color};

use std::borrow::Borrow;
use std::ops::{Add, AddAssign, BitOr, BitOrAssign, Sub, SubAssign};
use std::mem;

use self::color::Color;

#[derive(Copy, Clone, Default, Debug)]
pub struct TermStyle {
    pub(crate) attrs: [Option<Attr>; 10],
}

gen_idents!(bold, dim, blink, reverse, secure, italic, underline, standout, fg, bg,
            has_exact, has_variant, unset_exact, unset_variant, or, add);

// Public: methods for Attr variants
impl TermStyle {
    gen_attr_fns!(
        [bold, Bold],
        [dim, Dim],
        [blink, Blink],
        [reverse, Reverse],
        [secure, Secure]
    );

    gen_attr_fns!(
        [italic, Italic, bool],
        [underline, Underline, bool],
        [standout, Standout, bool],
        [fg, ForegroundColor, Color],
        [bg, BackgroundColor, Color]
    );
}

// Internal: use carefully
impl TermStyle {
    // Append attr at the position of first None, regardless
    // of what the array contents are.
    fn _append_attr(&mut self, attr: Attr) {
        let first_none = self.attrs.iter()
            .position(|&g_attr| g_attr == None)
            .expect("should never happen");

        self.attrs[first_none] = Some(attr);
    }

    // Find the position of an attr match if any.
    // Will match by variant if exact=false
    fn _attr_match_pos(&self, attr: Attr, exact: bool) -> Option<usize> {
        if exact {
            self.attrs.iter().position(|&g_attr| g_attr == Some(attr))
        } else {
            self.attrs.iter().position(|&g_attr| {
                match g_attr {
                    None       => false,
                    Some(g_attr) => mem::discriminant(&g_attr) == mem::discriminant(&attr),
                }
            })
        }
    }

    // If no variant match is found, append attr
    // If a match is found, replace the match if
    // replace=true, otherwise, don't do anything.
    fn _add_attr(&mut self, attr: Attr, replace: bool) {
        let variant_match_pos = self._attr_match_pos(attr, false);

        match (variant_match_pos, replace) {
            (None, _) => self._append_attr(attr),
            (Some(pos), true) => self.attrs[pos] = Some(attr),
            _ => (),
        };
    }

    // Remove an attr if a match is found (replace with None).
    // Will match by variant if exact=false
    fn _remove_attr(&mut self, attr: Attr, exact: bool) {
        let match_pos = self._attr_match_pos(attr, exact);

        if let Some(pos) = match_pos {
            self.attrs[pos] = None;
        }
    }
}

// Public: attr methods
impl TermStyle {
    pub fn has_exact_attr(&self , attr: Attr) -> bool {
        self._attr_match_pos(attr, true).is_some()
    }

    pub fn has_variant_attr(&self , attr: Attr) -> bool {
        self._attr_match_pos(attr, false).is_some()
    }

    pub fn unset_exact_attr(&mut self, attr: Attr) {
        self._remove_attr(attr, true);
    }

    pub fn unset_variant_attr(&mut self, attr: Attr) {
        self._remove_attr(attr, false);
    }

    pub fn or_attr(&mut self, attr: Attr) {
        // replace = false
        self._add_attr(attr, false);
    }

    pub fn add_attr(&mut self, attr: Attr) {
        // replace = true
        self._add_attr(attr, true);
    }

    gen_with_fn!(attr, without_exact_attr, unset_exact_attr);
    gen_with_fn!(attr, without_variant_attr, unset_variant_attr);
    gen_with_fn!(attr, with_attr, add_attr);
    gen_with_fn!(attr, with_ored_attr, or_attr);
}

// Public: attrs methods
impl TermStyle {
    gen_from_attr_fns!(has_attrs, has_exact, has_variant);
    gen_from_attr_fns!(attrs, unset_exact, unset_variant, or, add);

    gen_with_fn!(attrs, without_exact_attrs, unset_exact_attrs);
    gen_with_fn!(attrs, without_variant_attrs, unset_variant_attrs);
    gen_with_fn!(attrs, with_attrs, add_attrs);
    gen_with_fn!(attrs, with_ored_attrs, or_attrs);
}

// Public: style methods
impl TermStyle {
    gen_from_attr_fns!(has_style, has_exact, has_variant);
    gen_from_attr_fns!(style, unset_exact, unset_variant, or, add);

    gen_with_fn!(style, with_style, add_style);
    gen_with_fn!(style, with_ored_style, or_style);
    gen_with_fn!(style, without_exact_style, unset_exact_style);
    gen_with_fn!(style, without_variant_style, unset_variant_style);

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn eq_style(&self, other: impl Into<Self>) -> bool {
        let other = other.into();
        self.has_exact_style(other) && other.has_exact_style(*self)
    }

    pub fn eq_variant_style(&self, other: impl Into<Self>) -> bool {
        let other = other.into();
        self.has_variant_style(other) && other.has_variant_style(*self)
    }
}

impl PartialEq for TermStyle {
    fn eq(&self, other: &Self) -> bool {
        self.eq_style(*other)
    }
}

impl<A: Borrow<[Attr]>> From<A> for TermStyle {
    fn from(attrs: A) -> Self {
        Self::default().with_attrs(&attrs)
    }
}

impl<I: Into<Self>> BitOr<I> for TermStyle {
    type Output = Self;
    fn bitor(self, other: I) -> Self {
        self.with_ored_style(other.into())
    }
}

impl<I: Into<Self>> BitOrAssign<I> for TermStyle {
    fn bitor_assign(&mut self, other: I) {
        self.or_style(other.into());
    }
}

impl<I: Into<Self>> Add<I> for TermStyle {
    type Output = Self;
    fn add(self, other: I) -> Self {
        self.with_style(other.into())
    }
}

impl<I: Into<Self>> AddAssign<I> for TermStyle {
    fn add_assign(&mut self, other: I) {
        self.add_style(other.into());
    }
}

impl<I: Into<Self>> Sub<I> for TermStyle {
    type Output = Self;
    fn sub(self, other: I) -> Self {
        self.without_exact_style(other.into())
    }
}

impl<I: Into<Self>> SubAssign<I> for TermStyle {
    fn sub_assign(&mut self, other: I) {
        self.unset_exact_style(other.into());
    }
}
