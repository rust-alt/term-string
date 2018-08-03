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
                m["has" $t] = has_ $t;
                m["has_exact" $t] = has_exact_ $t;
                m["with" $t] = with_ $t;
                m["unset" $t] = unset_ $t;
                m["unset_exact" $t] = unset_exact_ $t;
                m["without" $t] = without_ $t;
                m["without_exact" $t] = without_exact_ $t;
                m["with_ored" $t] = with_ored_ $t;
                m["attr" $t] = $t _attr;
                m["attrs" $t] = $t _attrs;
                m["style" $t] = $t _style;
        )* }
    );
}

macro_rules! gen_attr_fns {
    ($([$t:ident, $v:ident]),*) => (
        m! { $(
                pub fn $t() -> Self {
                    Self::from([Attr::$v])
                }

                pub fn "has" $t(&self) -> bool {
                    self.has_exact_attr(Attr::$v)
                }

                pub fn "unset" $t(&mut self){
                    self.unset_exact_attr(Attr::$v);
                }

                pub fn "with" $t(self) -> Self {
                    self.with_attr(Attr::$v)
                }

                pub fn "without" $t(self) -> Self {
                    self.without_exact_attr(Attr::$v)
                }
            )* }
    );

    ($([$t:ident, $v:ident, $arg_ty:ty]),*) => (
        m! { $(
                pub fn $t(arg: $arg_ty) -> Self {
                    Self::from([Attr::$v(arg)])
                }

                pub fn "has_exact" $t(&self, arg: $arg_ty) -> bool {
                    self.has_exact_attr(Attr::$v(arg))
                }

                pub fn "has" $t(&self) -> bool {
                    self.has_variant_attr(Attr::$v(Default::default()))
                }

                pub fn "unset" $t(&mut self) {
                    self.unset_variant_attr(Attr::$v(Default::default()))
                }

                pub fn "unset_exact" $t(&mut self, arg: $arg_ty) {
                    self.unset_exact_attr(Attr::$v(arg))
                }

                pub fn "with" $t(self, arg: $arg_ty) -> Self {
                    self.with_attr(Attr::$v(arg))
                }

                pub fn "with_ored" $t(self, arg: $arg_ty) -> Self {
                    self.with_ored_attr(Attr::$v(arg))
                }

                pub fn "without_exact" $t(self, arg: $arg_ty) -> Self {
                    self.without_exact_attr(Attr::$v(arg))
                }

                pub fn "without" $t(self) -> Self {
                    self.without_variant_attr(Attr::$v(Default::default()))
                }
            )* }
    );


}

macro_rules! gen_with_fn {
    (attr, $t:ident, $b:ident) => {
        gen_with_fn_with_doc!(
            attr,
            concat!("The owning version of [`", stringify!($b), "`]."),
            concat!("[`", stringify!($b), "`]: TermStyle::", stringify!($b)),
            $t,
            $b
        );
    };
    (attrs, $t:ident, $b:ident) => {
        gen_with_fn_with_doc!(
            attrs,
            concat!("The owning version of [`", stringify!($b), "`]."),
            concat!("[`", stringify!($b), "`]: TermStyle::", stringify!($b)),
            $t,
            $b
        );
    };
    (style, $t:ident, $b:ident) => {
        gen_with_fn_with_doc!(
            style,
            concat!("The owning version of [`", stringify!($b), "`]."),
            concat!("[`", stringify!($b), "`]: TermStyle::", stringify!($b)),
            $t,
            $b
        );
    };
}

macro_rules! gen_with_fn_with_doc {
    (attr, $doc:expr, $doc2:expr, $t:ident, $b:ident) => (
        #[doc = $doc]
        ///
        #[doc = $doc2]
        pub fn $t(mut self, attr: Attr) -> Self {
            self.$b(attr);
            self
        }
    );

    (attrs, $doc:expr, $doc2:expr, $t:ident, $b:ident) => (
        #[doc = $doc]
        ///
        #[doc = $doc2]
        pub fn $t<A: Borrow<[Attr]>>(mut self, attrs: &A) -> Self {
            self.$b(attrs);
            self
        }
    );

    (style, $doc:expr, $doc2:expr, $t:ident, $b:ident) => (
        #[doc = $doc]
        ///
        #[doc = $doc2]
        pub fn $t<IS: Into<Self>>(mut self, other: IS) -> Self {
            self.$b(other);
            self
        }
    );
}

macro_rules! gen_from_attr_fns {
    (attrs, $($t:ident),*) => (
        m! { $(
                pub fn "attrs" $t<A: Borrow<[Attr]>>(&mut self, attrs: &A) {
                    attrs.borrow().iter().for_each(|&attr| self."attr" $t(attr));
                }
        )* }
    );

    (has_attrs, $($t:ident),*) => (
        m! { $(
                pub fn "attrs" $t<A: Borrow<[Attr]>>(&mut self, attrs: &A) -> bool {
                    attrs.borrow().iter()
                        .map(|&attr| self."attr" $t(attr))
                        .find(|&has| has == false).is_none()
                }
        )* }
    );

    (style, $($t:ident),*) => (
        m! { $(
                pub fn "style" $t<IS: Into<Self>>(&mut self, other: IS) {
                    other.into().attrs.iter()
                        .filter_map(|&attr| attr)
                        .for_each(|attr| self."attr" $t(attr));
                }
        )* }
    );

    (has_style, $($t:ident),*) => (
        m! { $(
                pub fn "style" $t<IS: Into<Self>>(&self, other: IS) -> bool {
                    other.into().attrs.iter()
                        .filter_map(|&attr| attr)
                        .map(|attr| self."attr" $t(attr))
                        .find(|&has| has == false).is_none()
                }
        )* }
    );
}
