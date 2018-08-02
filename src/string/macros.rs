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
                m[$t "plain"] = $t _plain;
                m[$t "styled"] = $t _styled;
                m[$t "ln_plain"] = $t ln_plain;
                m[$t "ln_styled"] = $t ln_styled;
                m[$t "ln"] = $t ln;
                m[$t "isatty"] = $t _isatty;
        )* }
    );
}

macro_rules! gen_print_fns {
    ($dev:ident, $print:ident) => {
        m! {
            pub fn $print "plain"(&self) {
                #[cfg(windows)]
                self.write_plain(&|| io::$dev());
                #[cfg(not(windows))]
                {
                    let $dev = io::$dev();
                    self.write_plain(&|| $dev.lock());
                }
            }

            pub fn $print "styled"(&self) {
                #[cfg(windows)]
                self.write_styled(&|| io::$dev());
                #[cfg(not(windows))]
                {
                    let $dev = io::$dev();
                    self.write_styled(&|| $dev.lock());
                }
            }

            pub fn $print(&self) {
                if isatty::$dev "isatty"() {
                    self.$print "styled"()
                } else {
                    self.$print "plain"()
                }
            }

            pub fn $print "ln_plain"(&self) {
                self.$print "plain"();
                Self::from("\n").$print "plain"();
            }

            pub fn $print "ln_styled"(&self) {
                self.$print "styled"();
                Self::from("\n").$print "plain"();
            }

            pub fn $print "ln"(&self) {
                self.$print();
                Self::from("\n").$print "plain"();
            }
        }
    };
}
