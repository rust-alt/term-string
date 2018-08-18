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
                m_has["has" $t] = has_ $t;
                m_has["has_exact" $t] = has_exact_ $t;
                m_unset["unset" $t] = unset_ $t;
                m_unset["without" $t] = without_ $t;
                m_unset["unset_exact" $t] = unset_exact_ $t;
                m_unset["without_exact" $t] = without_exact_ $t;
                m_op["add" $t] = add_ $t;
                m_op["or" $t] = or_ $t;
                m_op["with" $t] = with_ $t;
                m_op["with_ored" $t] = with_ored_ $t;
                m_a_s["attr" $t] = $t _attr;
                m_a_s["style" $t] = $t _style;
        )* }
    );
}

macro_rules! gen_attr_fns {
    ($([$t:ident, $v:ident]),*) => (
        $(
            gen_fn_with_doc!(
                concat!("Create a new [`TermStyle`] with [`Attr::", stringify!($v), "`] set.\n\n",
                "This is equivalent to `TermStyle::from([`[`Attr::", stringify!($v), "`]`])`."),
                pub fn $t() -> Self {
                    Self::from([Attr::$v])
                }
            );
        )*

        m_has! { $(

                gen_fn_with_doc!(
                    concat!("Check if [`Attr::", stringify!($v), "`] is set in style."),
                    pub fn "has" $t(&self) -> bool {
                        self.has_exact_attr(Attr::$v)
                    }
                );
            )* }

        m_op! { $(
                gen_fn_with_doc!(
                    concat!("Set/Add [`Attr::", stringify!($v), "`] to style."),
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
            )* }
        m_unset! { $(
                gen_fn_with_doc!(
                    concat!("Unset/Remove [`Attr::", stringify!($v), "`] from style."),
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
        $(
            gen_fn_with_doc!(
                concat!("Create a new [`TermStyle`] with [`Attr::", stringify!($v), "`]`(arg)` set.\n\n",
                "This is equivalent to `TermStyle::from([`[`Attr::", stringify!($v), "`]`(arg) ])`."),
                pub fn $t(arg: $arg_ty) -> Self {
                    Self::from([Attr::$v(arg)])
                }
            );
        )*

        m_has! { $(
                gen_fn_with_doc!(
                    concat!("Check if [`Attr::", stringify!($v), "`]`(val)` is set in style.\n",
                            "where `val` can be any value of [`", stringify!($arg_ty) ,"`]."),
                    pub fn "has" $t(&self) -> bool {
                        self.has_variant_attr(Attr::$v(Default::default()))
                    }
                );

                gen_fn_with_doc!(
                    concat!("Check if [`Attr::", stringify!($v), "`]`(arg)` is set in style."),
                    pub fn "has_exact" $t(&self, arg: $arg_ty) -> bool {
                        self.has_exact_attr(Attr::$v(arg))
                    }
                );
            )* }
        m_op! { $(
                gen_fn_with_doc!(
                    concat!("Set/Add [`Attr::", stringify!($v), "`]`(arg)` to style."),
                    pub fn "add" $t(&mut self, arg: $arg_ty) {
                        self.add_attr(Attr::$v(arg));
                    }
                );

                chaining_fn!(
                    TermStyle, "add" $t,
                    pub fn "with" $t(self, arg: $arg_ty) -> Self {
                        self.with_attr(Attr::$v(arg))
                    }
                );

                gen_fn_with_doc!(
                    concat!("Set/Add [`Attr::", stringify!($v), "`]`(arg)` to style,\n",
                    "if [`Attr::", stringify!($v), "`] is not already set."),
                    pub fn "or" $t(&mut self, arg: $arg_ty) {
                        self.or_attr(Attr::$v(arg))
                    }
                );

                chaining_fn!(
                    TermStyle, "or" $t,
                    pub fn "with_ored" $t(self, arg: $arg_ty) -> Self {
                        self.with_ored_attr(Attr::$v(arg))
                    }
                );
            )* }
        m_unset! { $(
                gen_fn_with_doc!(
                    concat!("Unset/Remove [`Attr::", stringify!($v), "`]`(val)` from style.\n",
                            "where `val` can be any value of [`", stringify!($arg_ty) ,"`]."),
                    pub fn "unset" $t(&mut self) {
                        self.unset_variant_attr(Attr::$v(Default::default()))
                    }
                );

                chaining_fn!(
                    TermStyle, "unset" $t,
                    pub fn "without" $t(self) -> Self {
                        self.without_variant_attr(Attr::$v(Default::default()))
                    }
                );

                gen_fn_with_doc!(
                    concat!("Unset/Remove [`Attr::", stringify!($v), "`]`(arg)` from style."),
                            pub fn "unset_exact" $t(&mut self, arg: $arg_ty) {
                                self.unset_exact_attr(Attr::$v(arg))
                            }
                );

                chaining_fn!(
                    TermStyle, "unset_exact" $t,
                    pub fn "without_exact" $t(self, arg: $arg_ty) -> Self {
                        self.without_exact_attr(Attr::$v(arg))
                    }
                );
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
        m_a_s! { $(
                pub fn "style" $t<IS>(&mut self, other: IS) where IS: Into<Self> {
                    other.into().attrs.iter()
                        .filter_map(|&attr| attr)
                        .for_each(|attr| self."attr" $t(attr));
                }
        )* }
    );

    (has_style, $($t:ident),*) => (
        m_a_s! { $(
                pub fn "style" $t<IS>(&self, other: IS) -> bool where IS: Into<Self> {
                    other.into().attrs.iter()
                        .filter_map(|&attr| attr)
                        .map(|attr| self."attr" $t(attr))
                        .find(|&has| !has).is_none()
                }
        )* }
    );
}
