/*
    This file is a part of term-string.

    Copyright (C) 2018 Mohammad AlSaleh <CE.Mohammad.AlSaleh at gmail.com>
    https://github.com/rust-alt/term-string

    This Source Code Form is subject to the terms of the Mozilla Public
    License, v. 2.0. If a copy of the MPL was not distributed with this
    file, You can obtain one at <http://mozilla.org/MPL/2.0/>.
*/

macro_rules! chaining_fn {
    ($ty:ty, $br:ident, $($tt:tt)*) => (
        gen_chaining_fn_with_doc!(concat!("The chaining equivalent of [`", stringify!($br), "()`].\n\n",
        "[`", stringify!($br), "()`]: ", stringify!($ty), "::", stringify!($br)), $($tt)*);
    )
}

macro_rules! gen_chaining_fn_with_doc {
    ($doc:expr, $($tt:tt)*) => (
        #[doc = $doc]
        $($tt)*
    )
}
