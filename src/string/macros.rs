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
            gen_fn_with_doc!(
                concat!(
                    "Write [`TermString`] to `", stringify!($dev), "` without styling.\n\n",
                    "# Examples\n\n",
                    "``` rust\n",
                    "# use term_string::{TermString, TermStyle};\n",
                    "let bold = TermStyle::bold();\n",
                    "let ts = TermString::new(bold, \"some bold text\");\n\n",
                    "// This will write \"some bold text\" to `", stringify!($dev), "` without\n",
                    "// any formatting, so not really bold.\n",
                    "ts.", stringify!($print "plain"), "();\n",
                    "```"
                ),
                pub fn $print "plain"(&self) {
                    #[cfg(windows)]
                    self.write_plain(io::$dev());
                    #[cfg(not(windows))]
                    {
                        let $dev = io::$dev();
                        self.write_plain($dev.lock());
                    }
                }
            );

            gen_fn_with_doc!(
                concat!(
                    "Print [`TermString`] to `", stringify!($dev), "` with styling.\n\n",
                    "# Note\n\n",
                    "`", stringify!($dev), "` doesn't have to be an actual tty.\n\n",
                    "Check out [`", stringify!($print), "()`] below, where `", stringify!($dev), "`\n",
                    "is checked before styled output is printed.\n\n",
                    "# Examples\n",
                    "``` rust\n",
                    "# use term_string::{TermString, TermStyle};\n",
                    "let bold = TermStyle::bold();\n",
                    "let ts = TermString::new(bold, \"some bold text\");\n\n",
                    "// This will write \"some bold text\" to ", stringify!($dev), "\n",
                    "// as bold text.\n",
                    "ts.", stringify!($print "styled"), "();\n",
                    "```\n\n",
                    "[`", stringify!($print), "()`]: TermString::", stringify!($print)
                ),
                pub fn $print "styled"(&self) {
                    #[cfg(windows)]
                    self.write_styled(io::$dev());
                    #[cfg(not(windows))]
                    {
                        let $dev = io::$dev();
                        self.write_styled($dev.lock());
                    }
                }
            );

            gen_fn_with_doc!(
                concat!(
                    "Print [`TermString`] to `", stringify!($dev), "` with styling\n",
                    "if `", stringify!($dev), "` is a tty, and without if it's not.\n\n",
                    "# Examples\n",
                    "``` rust\n",
                    "# use term_string::{TermString, TermStyle};\n",
                    "let bold = TermStyle::bold();\n",
                    "let ts = TermString::new(bold, \"some bold text\");\n\n",
                    "// This will write \"some bold text\" to ", stringify!($dev), " as bold\n",
                    "// text if ", stringify!($dev), " is a tty, and without any formatting if not.\n",
                    "ts.", stringify!($print), "();\n",
                    "```"
                ),
                pub fn $print(&self) {
                    if isatty::$dev "isatty"() {
                        self.$print "styled"()
                    } else {
                        self.$print "plain"()
                    }
                }
            );

            gen_fn_with_doc!(
                concat!(
                    "The same as [`", stringify!($print "plain"), "()`], but with a newline printed at the end.\n\n",
                    "[`", stringify!($print "plain"), "()`]: TermString::", stringify!($print "plain")
                ),
                pub fn $print "ln_plain"(&self) {
                    self.$print "plain"();
                    Self::from("\n").$print "plain"();
                }
            );


            gen_fn_with_doc!(
                concat!(
                    "The same as [`", stringify!($print "styled"), "()`], but with a newline printed at the end.\n\n",
                    "[`", stringify!($print "styled"), "()`]: TermString::", stringify!($print "styled")
                ),
                pub fn $print "ln_styled"(&self) {
                    self.$print "styled"();
                    Self::from("\n").$print "plain"();
                }
            );

            gen_fn_with_doc!(
                concat!(
                    "The same as [`", stringify!($print), "()`], but with a newline printed at the end.\n\n",
                    "[`", stringify!($print), "()`]: TermString::", stringify!($print)
                ),
                pub fn $print "ln"(&self) {
                    self.$print();
                    Self::from("\n").$print "plain"();
                }
            );
        }
    };
}
