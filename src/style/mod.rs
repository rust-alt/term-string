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

pub use term::{color, Attr};

use std::borrow::Borrow;
use std::mem;
use std::ops::{Add, AddAssign, BitOr, BitOrAssign, Sub, SubAssign};

use self::color::Color;

#[derive(Copy, Clone, Default, Debug)]
/// TODO with ops
pub struct TermStyle {
    pub(crate) attrs: [Option<Attr>; 10],
}

gen_idents!(
    bold,
    dim,
    blink,
    reverse,
    secure,
    italic,
    underline,
    standout,
    fg,
    bg,
    has_exact,
    has_variant,
    unset_exact,
    unset_variant,
    or,
    add
);

/// Convenient methods for setting, unsetting, and checking [`Attr`] variants
/// in a [`TermStyle`] variable.
///
/// This block has helper methods for [`Attr`] variants with no data:
///
/// (*`bold`*, *`dim`*, *`blink`*, *`reverse`*, *`secure`*)
///
/// # Examples
/// ``` rust
/// # use term_string::TermStyle;
/// let mut style = TermStyle::bold();
/// assert!(style.has_bold());
/// style.unset_bold();
/// assert!(!style.has_bold());
/// ```
///
impl TermStyle {
    gen_attr_fns!(
        [bold, Bold],
        [dim, Dim],
        [blink, Blink],
        [reverse, Reverse],
        [secure, Secure]
    );
}

/// Convenient methods for setting, unsetting, and checking [`Attr`] variants
/// in a [`TermStyle`] variable.
///
/// This block has helper methods for [`Attr`] variants with `bool` data:
///
/// (*`italic`*, *`underline`*, *`standout`*)
///
/// # Note
///
/// Unlike the attribute variants in the above block, those attributes have
/// `bool` data. This is because those capabilities were a late addition to
/// the `terminfo` database. And when they were added, they were added in
/// pairs (enter capability mode, exit capability mode).
///
/// There is no reason and no need to set any of those attributes with `false`
/// here, as styles are fully reset between writes/prints. The API is still fully
/// exposed to stay close and introduce no magic over what the [`term`] crate
/// exposes ([`Attr`] is a re-export of [`term`]::Attr).
///
/// # Examples
/// ``` rust
/// # use term_string::TermStyle;
/// let mut style = TermStyle::underline(true);
/// // Returns true if underline is set, to true or false.
/// assert!(style.has_underline());
/// assert!(style.has_exact_underline(true));
/// assert!(!style.has_exact_underline(false));
/// // style.unset_exact_underline(true);
/// // Unsets whether underline is true or false.
/// style.unset_underline();
/// assert!(!style.has_underline());
/// ```
///
impl TermStyle {
    gen_attr_fns!(
        [italic, Italic, bool],
        [underline, Underline, bool],
        [standout, Standout, bool]
    );
}

/// Convenient methods for setting, unsetting, and checking [`Attr`] variants
/// in a [`TermStyle`] variable.
///
/// This block has helper methods for [`Attr`] variants with [`Color`] data:
///
/// (*`fg`*, *`bg`*)
///
/// # Examples
/// ``` rust
/// use term_string::{TermStyle, color};
///
/// let mut style = TermStyle::fg(color::BLUE);
/// style += TermStyle::bg(color::WHITE);
/// // ==========
/// assert!(style.has_fg());
/// assert!(style.has_bg());
/// // ==========
/// assert!(style.has_exact_fg(color::BLUE));
/// assert!(style.has_exact_bg(color::WHITE));
/// // ==========
/// assert!(!style.has_exact_fg(color::RED));
/// assert!(!style.has_exact_bg(color::GREEN));
/// // ==========
/// style.unset_exact_fg(color::RED); // no effect
/// style.unset_exact_bg(color::GREEN); // no effect
/// assert!(!style.has_exact_fg(color::RED));
/// assert!(!style.has_exact_bg(color::GREEN));
/// // ==========
/// style.unset_fg();
/// style.unset_bg();
/// assert!(!style.has_fg());
/// assert!(!style.has_bg());
/// ```
///
impl TermStyle {
    gen_attr_fns!([fg, ForegroundColor, Color], [bg, BackgroundColor, Color]);
}

// Internal: use carefully
impl TermStyle {
    // Append attr at the position of first None, regardless
    // of what the array contents are.
    fn _append_attr(&mut self, attr: Attr) {
        let first_none = self
            .attrs
            .iter()
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
            self.attrs.iter().position(|&g_attr| match g_attr {
                None => false,
                Some(g_attr) => mem::discriminant(&g_attr) == mem::discriminant(&attr),
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

/// Methods that take [`Attr`] as an argument.
/// Note that [`Attr`] is a [`Copy`] enum type.
impl TermStyle {
    /// [`TermStyle`] has attr set. Exact is referring to
    /// attr's data, if exists, being included in
    /// the check.
    ///
    /// # Examples
    /// ``` rust
    /// use term_string::{TermStyle, Attr};
    ///
    /// let style = TermStyle::underline(true);
    /// assert!(style.has_exact_attr(Attr::Underline(true)));
    /// assert!(!style.has_exact_attr(Attr::Underline(false)));
    /// ```
    pub fn has_exact_attr(&self, attr: Attr) -> bool {
        self._attr_match_pos(attr, true).is_some()
    }

    /// [`TermStyle`] has attr set. Variant is referring to
    /// attr's data, if exists, being excluded from
    /// the check.
    ///
    /// # Examples
    /// ``` rust
    /// use term_string::{TermStyle, Attr};
    ///
    /// let style = TermStyle::underline(true);
    /// assert!(style.has_variant_attr(Attr::Underline(true)));
    /// assert!(style.has_variant_attr(Attr::Underline(false)));
    /// ```
    pub fn has_variant_attr(&self, attr: Attr) -> bool {
        self._attr_match_pos(attr, false).is_some()
    }

    /// Unset/Remove the exact [`Attr`] from [`TermStyle`].
    ///
    /// # Examples
    /// ``` rust
    /// use term_string::{TermStyle, Attr};
    ///
    /// let mut style = TermStyle::underline(true);
    ///
    /// // this does nothing, no exact match
    /// style.unset_exact_attr(Attr::Underline(false));
    /// assert_ne!(style, TermStyle::default());
    ///
    /// // this unsets underline, exact match
    /// style.unset_exact_attr(Attr::Underline(true));
    /// assert_eq!(style, TermStyle::default());
    /// ```
    pub fn unset_exact_attr(&mut self, attr: Attr) {
        self._remove_attr(attr, true);
    }

    /// Unset/Remove the variant [`Attr`] from [`TermStyle`].
    ///
    /// # Examples
    /// ``` rust
    /// use term_string::{TermStyle, Attr};
    ///
    /// let mut style = TermStyle::underline(true);
    ///
    /// // this unsets underline, even without exact match
    /// style.unset_variant_attr(Attr::Underline(false));
    /// assert_eq!(style, TermStyle::default());
    /// ```
    pub fn unset_variant_attr(&mut self, attr: Attr) {
        self._remove_attr(attr, false);
    }

    /// Set/Add attr to [`TermStyle`], unless the same variant
    /// has been already set.
    ///
    /// # Examples
    /// ``` rust
    /// use term_string::{TermStyle, Attr, color};
    ///
    /// let mut style = TermStyle::default();
    ///
    /// // Add red background
    /// style.or_attr(Attr::BackgroundColor(color::RED));
    /// assert!(style.has_exact_bg(color::RED));
    ///
    /// // Add green background if background is not already set
    /// style.or_attr(Attr::BackgroundColor(color::GREEN));
    /// // Since background was already set, it's still red
    /// assert!(style.has_exact_bg(color::RED));
    /// ```
    pub fn or_attr(&mut self, attr: Attr) {
        // replace = false
        self._add_attr(attr, false);
    }

    /// Set/Add attr to [`TermStyle`], overriding the same variant
    /// if it was already set.
    ///
    /// # Examples
    /// ``` rust
    /// use term_string::{TermStyle, Attr, color};
    ///
    /// let mut style = TermStyle::default();
    ///
    /// // Add red background
    /// style.add_attr(Attr::BackgroundColor(color::RED));
    /// assert!(style.has_exact_bg(color::RED));
    ///
    /// // Add green background, overriding already set background
    /// style.add_attr(Attr::BackgroundColor(color::GREEN));
    /// // background will always be green after the above line
    /// assert!(style.has_exact_bg(color::GREEN));
    /// ```
    pub fn add_attr(&mut self, attr: Attr) {
        // replace = true
        self._add_attr(attr, true);
    }

    gen_with_fn!(attr, without_exact_attr, unset_exact_attr);
    gen_with_fn!(attr, without_variant_attr, unset_variant_attr);
    gen_with_fn!(attr, with_attr, add_attr);
    gen_with_fn!(attr, with_ored_attr, or_attr);
}

/// Methods that take `Into<Self>` as an argument. So you can either
/// pass [`TermStyle`], or [`Attr`] arrays or slices.
///
/// These methods mirror the behavior of the methods above that take
/// a single [`Attr`] argument, except they apply to all attributes set
/// in the style passed as argument.
///
/// # Examples
///
/// ``` rust
/// use term_string::{TermStyle, Attr};
///
/// let st1 = TermStyle::default()
///   .with_attr(Attr::Bold)
///   .with_attr(Attr::Underline(true));
///
/// let st2 = TermStyle::default()
///   .with_style([Attr::Bold, Attr::Underline(true)]);
///
/// assert_eq!(st1, st2);
/// assert!(st1.has_exact_style([Attr::Bold, Attr::Underline(true)]));
/// assert!(st2.has_exact_style(st1));
///
/// ```
///
impl TermStyle {
    gen_from_attr_fns!(has_style, has_exact, has_variant);
    gen_from_attr_fns!(style, unset_exact, unset_variant, or, add);

    gen_with_fn!(style, with_style, add_style);
    gen_with_fn!(style, with_ored_style, or_style);
    gen_with_fn!(style, without_exact_style, unset_exact_style);
    gen_with_fn!(style, without_variant_style, unset_variant_style);
}

/// Other methods
impl TermStyle {
    /// Resets style to default.
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Checks if both styles have the same exact attributes set.
    /// This is used for implementing [`PartialEq`], so you should
    /// probably use that instead.
    ///
    /// # Examples
    ///
    /// ``` rust
    /// use term_string::{TermStyle, Attr};
    ///
    /// let st1 = TermStyle::bold() + TermStyle::underline(true);
    /// let st2 = st1 + TermStyle::reverse();
    /// assert!(st1.eq_style([Attr::Underline(true), Attr::Bold]));
    /// // Needless to say, subset != equal
    /// assert!(!st2.eq_style(st1));
    /// ```
    pub fn eq_style<IS>(&self, other: IS) -> bool
    where
        IS: Into<Self>,
    {
        let other = other.into();
        self.has_exact_style(other) && other.has_exact_style(*self)
    }

    /// Checks if both styles have the same exact attribute variants set.
    ///
    /// # Examples
    ///
    /// ``` rust
    /// use term_string::{TermStyle, Attr};
    ///
    /// let st1 = TermStyle::bold() + TermStyle::underline(true);
    /// let st2 = st1 + TermStyle::reverse();
    /// assert!(st1.eq_variant_style([Attr::Underline(true), Attr::Bold]));
    /// // Same variant, different data still counts as equal
    /// assert!(st1.eq_variant_style([Attr::Underline(false), Attr::Bold]));
    /// ```
    pub fn eq_variant_style<IS>(&self, other: IS) -> bool
    where
        IS: Into<Self>,
    {
        let other = other.into();
        self.has_variant_style(other) && other.has_variant_style(*self)
    }
}

impl PartialEq for TermStyle {
    fn eq(&self, other: &Self) -> bool {
        self.eq_style(*other)
    }
}

/// Get a [`TermStyle`] from [`Attr`] arrays or slices.
impl<A> From<A> for TermStyle
where
    A: Borrow<[Attr]>,
{
    fn from(attrs: A) -> Self {
        let mut style = Self::default();
        attrs.borrow().iter().for_each(|&a| style.add_attr(a));
        style
    }
}

/// Check out [`or_style()`] and [`or_attr()`].
///
/// [`or_style()`]: TermStyle::or_style
/// [`or_attr()`]: TermStyle::or_attr
impl<IS> BitOr<IS> for TermStyle
where
    IS: Into<Self>,
{
    type Output = Self;
    fn bitor(self, other: IS) -> Self {
        self.with_ored_style(other.into())
    }
}

impl<IS> BitOrAssign<IS> for TermStyle
where
    IS: Into<Self>,
{
    fn bitor_assign(&mut self, other: IS) {
        self.or_style(other.into());
    }
}

/// Check out [`add_style()`] and [`add_attr()`].
///
/// [`add_style()`]: TermStyle::add_style
/// [`add_attr()`]: TermStyle::add_attr
impl<IS> Add<IS> for TermStyle
where
    IS: Into<Self>,
{
    type Output = Self;
    fn add(self, other: IS) -> Self {
        self.with_style(other.into())
    }
}

impl<IS> AddAssign<IS> for TermStyle
where
    IS: Into<Self>,
{
    fn add_assign(&mut self, other: IS) {
        self.add_style(other.into());
    }
}

/// Check out [`unset_exact_style()`] and [`unset_exact_attr()`].
///
/// [`unset_exact_style()`]: TermStyle::unset_exact_style
/// [`unset_exact_attr()`]: TermStyle::unset_exact_attr
impl<IS> Sub<IS> for TermStyle
where
    IS: Into<Self>,
{
    type Output = Self;
    fn sub(self, other: IS) -> Self {
        self.without_exact_style(other.into())
    }
}

impl<IS> SubAssign<IS> for TermStyle
where
    IS: Into<Self>,
{
    fn sub_assign(&mut self, other: IS) {
        self.unset_exact_style(other.into());
    }
}
