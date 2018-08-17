/*
    This file is a part of term-string.

    Copyright (C) 2018 Mohammad AlSaleh <CE.Mohammad.AlSaleh at gmail.com>
    https://github.com/rust-alt/term-string

    This Source Code Form is subject to the terms of the Mozilla Public
    License, v. 2.0. If a copy of the MPL was not distributed with this
    file, You can obtain one at <http://mozilla.org/MPL/2.0/>.
*/

// We do this separately because we can't invoke mashup! more than once:
// https://github.com/dtolnay/mashup/issues/5
// This macro will be nuked once concat_idents! becomes usable (if ever).
macro_rules! gen_idents {
    ($($t:ident),*) => (
        mashup! { $(
                m1["has" $t] = has_ $t;
                m1["unset" $t] = unset_ $t;
                m1["add" $t] = add_ $t;
                m1["with" $t] = with_ $t;
                m1["without" $t] = without_ $t;
                m2["has_exact" $t] = has_exact_ $t;
                m2["unset_exact" $t] = unset_exact_ $t;
                m2["without_exact" $t] = without_exact_ $t;
                m2["with_ored" $t] = with_ored_ $t;
                m3["attr" $t] = $t _attr;
                m3["style" $t] = $t _style;
        )* }
    );
}

macro_rules! gen_attr_fns {
    ($([$t:ident, $v:ident]),*) => (
        m1! { $(
                gen_fn_with_doc!(
                    concat!("Equivalent to `TermStyle::from([`[`Attr::", stringify!($v), "`]`])`."),
                    pub fn $t() -> Self {
                        Self::from([Attr::$v])
                    }
                );

                gen_fn_with_doc!(
                    concat!("Equivalent to self.[`has_exact_attr`]`(`[`Attr::", stringify!($v), "`]`)`.\n\n",
                    "Or self.[`has_variant_attr`]`(`[`Attr::", stringify!($v), "`]`)`.\n\n",
                    "The result is the same because [`Attr::", stringify!($v), "`] has no data.\n\n",
                    "[`has_exact_attr`]: TermStyle::has_exact_attr\n",
                    "[`has_variant_attr`]: TermStyle::has_variant_attr"),
                    pub fn "has" $t(&self) -> bool {
                        self.has_exact_attr(Attr::$v)
                    }
                );

                gen_fn_with_doc!(
                    concat!("Equivalent to self.[`add_attr`]`(`[`Attr::", stringify!($v), "`]`)`.\n\n",
                    "[`add_attr`]: TermStyle::add_attr"),
                    pub fn "add" $t(&mut self) {
                        self.add_attr(Attr::$v);
                    }
                );

                chaining_fn!(
                    TermStyle, "add" $t,
                    pub fn "with" $t(self) -> Self {
                        self.with_attr(Attr::$v)
                    }
                );

                gen_fn_with_doc!(
                    concat!("Equivalent to self.[`unset_exact_attr`]`(`[`Attr::", stringify!($v), "`]`)`.\n\n",
                    "Or self.[`unset_variant_attr`]`(`[`Attr::", stringify!($v), "`]`)`.\n\n",
                    "The result is the same because [`Attr::", stringify!($v), "`] has no data.\n\n",
                    "[`unset_exact_attr`]: TermStyle::unset_exact_attr\n",
                    "[`unset_variant_attr`]: TermStyle::unset_variant_attr"),
                    pub fn "unset" $t(&mut self) {
                        self.unset_exact_attr(Attr::$v);
                    }
                );

                chaining_fn!(
                    TermStyle, "unset" $t,
                    pub fn "without" $t(self) -> Self {
                        self.without_exact_attr(Attr::$v)
                    }
                );
            )* }
    );

    ($([$t:ident, $v:ident, $arg_ty:ty]),*) => (
        m1! { $(
                pub fn $t(arg: $arg_ty) -> Self {
                    Self::from([Attr::$v(arg)])
                }

                pub fn "has" $t(&self) -> bool {
                    self.has_variant_attr(Attr::$v(Default::default()))
                }

                pub fn "unset" $t(&mut self) {
                    self.unset_variant_attr(Attr::$v(Default::default()))
                }

                pub fn "add" $t(&mut self, arg: $arg_ty) {
                    self.add_attr(Attr::$v(arg));
                }

                pub fn "with" $t(self, arg: $arg_ty) -> Self {
                    self.with_attr(Attr::$v(arg))
                }

                pub fn "without" $t(self) -> Self {
                    self.without_variant_attr(Attr::$v(Default::default()))
                }
        )* }

        m2! { $(
                pub fn "has_exact" $t(&self, arg: $arg_ty) -> bool {
                    self.has_exact_attr(Attr::$v(arg))
                }

                pub fn "unset_exact" $t(&mut self, arg: $arg_ty) {
                    self.unset_exact_attr(Attr::$v(arg))
                }

                pub fn "with_ored" $t(self, arg: $arg_ty) -> Self {
                    self.with_ored_attr(Attr::$v(arg))
                }

                pub fn "without_exact" $t(self, arg: $arg_ty) -> Self {
                    self.without_exact_attr(Attr::$v(arg))
                }
            )* }
    );


}

macro_rules! gen_with_fn {
    (attr, $t:ident, $b:ident) => (
        chaining_fn!(TermStyle, $b,
                      pub fn $t(mut self, attr: Attr) -> Self {
                          self.$b(attr);
                          self
                      }
        );
    );

    (style, $t:ident, $b:ident) => (
        chaining_fn!(TermStyle, $b,
                      pub fn $t<IS>(mut self, other: IS) -> Self
                      where
                          IS: Into<Self>,
                      {
                          self.$b(other);
                          self
                      }
        );
    );
}

macro_rules! gen_from_attr_fns {
    (style, $($t:ident),*) => (
        m3! { $(
                pub fn "style" $t<IS>(&mut self, other: IS) where IS: Into<Self> {
                    other.into().attrs.iter()
                        .filter_map(|&attr| attr)
                        .for_each(|attr| self."attr" $t(attr));
                }
        )* }
    );

    (has_style, $($t:ident),*) => (
        m3! { $(
                pub fn "style" $t<IS>(&self, other: IS) -> bool where IS: Into<Self> {
                    other.into().attrs.iter()
                        .filter_map(|&attr| attr)
                        .map(|attr| self."attr" $t(attr))
                        .find(|&has| !has).is_none()
                }
        )* }
    );
}
