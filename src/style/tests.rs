/*
    This file is a part of term-string.

    Copyright (C) 2018 Mohammad AlSaleh <CE.Mohammad.AlSaleh at gmail.com>
    https://github.com/rust-alt/term-string

    This Source Code Form is subject to the terms of the Mozilla Public
    License, v. 2.0. If a copy of the MPL was not distributed with this
    file, You can obtain one at <http://mozilla.org/MPL/2.0/>.
*/

use style::color::RED;
use style::Attr;
use style::TermStyle as Sty;

#[test]
fn reset() {
    let mut style = Sty::underline(true) | Sty::bold() | Sty::secure();
    // None in the middle
    style -= Sty::bold();
    // reset
    style.reset();
    assert_eq!(style, Sty::default());
}

#[test]
fn unset_variant_attr() {
    let mut style1 = Sty::underline(false);
    let mut style2 = Sty::underline(true);
    style1.unset_variant_attr(Attr::Underline(false));
    style2.unset_variant_attr(Attr::Underline(false));
    assert_eq!(style1, Sty::default());
    assert_eq!(style2, Sty::default());
}

#[test]
fn without_variant_attr() {
    let style1 = Sty::bold() | Sty::underline(false) | Sty::secure();
    let style2 = Sty::bold() | Sty::underline(true) | Sty::secure();
    let style3 = Sty::bold() | Sty::secure();
    assert_eq!(style3, style1.without_variant_attr(Attr::Underline(false)));
    assert_eq!(style3, style2.without_variant_attr(Attr::Underline(false)));
}

#[test]
fn unset_exact_attr() {
    let mut style1 = Sty::underline(false);
    let mut style2 = Sty::underline(true);
    style1.unset_exact_attr(Attr::Underline(false));
    style2.unset_exact_attr(Attr::Underline(false));
    assert_eq!(style1, Sty::default());
    assert_eq!(style2, Sty::underline(true));
}

#[test]
fn without_exact_attr() {
    let style1 = Sty::bold() | Sty::underline(false) | Sty::secure();
    let style2 = Sty::bold() | Sty::underline(true) | Sty::secure();
    let style3 = Sty::bold() | Sty::secure();
    assert_eq!(style3, style1.without_exact_attr(Attr::Underline(false)));
    assert_eq!(style2, style2.without_exact_attr(Attr::Underline(false)));
}

#[test]
fn or_attrs() {
    let mut style1 = Sty::bold() | Sty::secure();
    let style2 = Sty::bold() | Sty::underline(false) | Sty::secure();
    style1.or_attrs(&[Attr::Underline(false)]);
    assert_eq!(style1, style2);
    style1.or_attrs(&[Attr::Underline(true)]);
    assert_eq!(style1, style2);
}

#[test]
fn add_attrs() {
    let mut style1 = Sty::bold() | Sty::secure();
    let style2 = Sty::bold() | Sty::underline(false) | Sty::secure();
    let style3 = Sty::bold() | Sty::underline(true) | Sty::secure();
    style1.add_attrs(&[Attr::Underline(false)]);
    assert_eq!(style1, style2);
    style1.add_attrs(&[Attr::Underline(true)]);
    assert_eq!(style1, style3);
}

#[test]
fn with_ored_attrs() {
    let style1 = Sty::bold() | Sty::secure();
    let style2 = Sty::bold() | Sty::underline(false) | Sty::secure();
    let style3 = Sty::bold() | Sty::underline(true) | Sty::secure();
    assert_eq!(style1.with_ored_attrs(&[Attr::Underline(false)]), style2);
    // We didn't append style1 itself, so Attr::Underline(true) will be appended
    assert_eq!(style1.with_ored_attrs(&[Attr::Underline(true)]), style3);
}

#[test]
fn with_attrs() {
    let style1 = Sty::bold() | Sty::secure();
    let style2 = Sty::bold() | Sty::underline(false) | Sty::secure();
    let style3 = Sty::bold() | Sty::underline(true) | Sty::secure();
    assert_eq!(style1.with_attrs(&[Attr::Underline(false)]), style2);
    assert_eq!(style1.with_attrs(&[Attr::Underline(true)]), style3);
}

#[test]
fn or_style() {
    let mut style1 = Sty::bold() | Sty::secure();
    let style2 = Sty::bold() | Sty::underline(false) | Sty::secure();
    style1.or_style(Sty::underline(false));
    assert_eq!(style1, style2);
    style1.or_style(Sty::underline(true));
    assert_eq!(style1, style2);
}

#[test]
fn add_style() {
    let mut style1 = Sty::bold() | Sty::secure();
    let style2 = Sty::bold() | Sty::underline(false) | Sty::secure();
    let style3 = Sty::bold() | Sty::underline(true) | Sty::secure();
    style1.add_style(Sty::underline(false));
    assert_eq!(style1, style2);
    style1.add_style(Sty::underline(true));
    assert_eq!(style1, style3);
}

#[test]
fn with_or_style() {
    let style1 = Sty::bold() | Sty::secure();
    let style2 = Sty::bold() | Sty::underline(false) | Sty::secure();
    let style3 = Sty::bold() | Sty::underline(true) | Sty::secure();
    assert_eq!(style1.with_ored_style(Sty::underline(false)), style2);
    // We didn't append style1 itself, so Attr::Underline(true) will be appended
    assert_eq!(style1.with_ored_style(Sty::underline(true)), style3);
}

#[test]
fn with_style() {
    let style1 = Sty::bold() | Sty::secure();
    let style2 = Sty::bold() | Sty::underline(false) | Sty::secure();
    let style3 = Sty::bold() | Sty::underline(true) | Sty::secure();
    assert_eq!(style1.with_style(Sty::underline(false)), style2);
    assert_eq!(style1.with_style(Sty::underline(true)), style3);
}

#[test]
fn unset_exact_style() {
    let mut style1 = Sty::bold() | Sty::underline(false) | Sty::secure();
    let mut style2 = Sty::bold() | Sty::underline(true) | Sty::secure();
    let style3 = Sty::bold() | Sty::secure();
    style1.unset_exact_style(Sty::underline(false));
    assert_eq!(style1, style3);
    // Only subtracts exact matches
    style2.unset_exact_style(Sty::underline(false));
    assert_ne!(style2, style3);
}

#[test]
fn unset_variant_style() {
    let mut style1 = Sty::bold() | Sty::underline(false) | Sty::secure();
    let mut style2 = Sty::bold() | Sty::underline(true) | Sty::secure();
    let style3 = Sty::bold() | Sty::secure();
    style1.unset_variant_style(Sty::underline(true));
    assert_eq!(style1, style3);
    style2.unset_variant_style(Sty::underline(false));
    assert_eq!(style2, style3);
}

#[test]
fn without_exact_style() {
    let style1 = Sty::bold() | Sty::underline(false) | Sty::secure();
    let style2 = Sty::bold() | Sty::underline(true) | Sty::secure();
    let style3 = Sty::bold() | Sty::secure();
    assert_eq!(style1.without_exact_style(Sty::underline(false)), style3);
    // Only subtracts exact matches
    assert_ne!(style2.without_exact_style(Sty::underline(false)), style3);
}

#[test]
fn without_variant_style() {
    let style1 = Sty::bold() | Sty::underline(false) | Sty::secure();
    let style2 = Sty::bold() | Sty::underline(true) | Sty::secure();
    let style3 = Sty::bold() | Sty::secure();
    assert_eq!(style1.without_variant_style(Sty::underline(true)), style3);
    assert_eq!(style2.without_variant_style(Sty::underline(false)), style3);
}

#[test]
fn ops_partial_eq() {
    let mut style1 = Sty::bold() | Sty::underline(false) | Sty::secure() | Sty::fg(RED);
    let mut style2 = Sty::bold() | Sty::secure() | Sty::underline(false) | Sty::fg(RED);
    let style3 = Sty::bold() | Sty::secure() | Sty::underline(true) | Sty::fg(RED);
    let style4 = Sty::secure() | Sty::bold() | Sty::fg(RED);
    assert_eq!(style1, style2);
    assert_ne!(style2, style3);
    assert_ne!(style1, style4);
    // style4 is a sub-set of style3
    assert_ne!(style4, style3);
    style1.unset_underline();
    style2.unset_underline();
    assert_eq!(style1, style2);
    assert_eq!(style1, style4);
}

#[test]
fn ops_from() {
    let style1 = Sty::from([Attr::Bold, Attr::Underline(false), Attr::Secure]);
    let style2 = Sty::from([Attr::Bold, Attr::Underline(true), Attr::Secure]);
    let style3 = Sty::from([Attr::Bold, Attr::Secure, Attr::Underline(true)]);
    assert_ne!(style1, style2);
    assert_ne!(style1, style3);
    assert_eq!(style2, style3);
}

#[test]
fn ops_bitor() {
    let style1 = Sty::secure() | Sty::fg(RED);
    let style2 = Sty::bold() | Sty::secure() | Sty::fg(RED);
    let style3 = Sty::bold() | Sty::underline(false) | Sty::secure() | Sty::fg(RED);
    assert_eq!(style1 | Sty::bold(), style2);
    assert_eq!(style2 | Sty::underline(false), style3);
    assert_eq!(style3 | Sty::underline(true), style3);
}

#[test]
fn ops_bitor_assign() {
    let mut style1 = Sty::secure() | Sty::fg(RED);
    let mut style2 = Sty::bold() | Sty::secure() | Sty::fg(RED);
    let mut style3 = Sty::bold() | Sty::underline(false) | Sty::secure() | Sty::fg(RED);
    style1 |= Sty::bold();
    assert_eq!(style1, style2);
    style2 |= Sty::underline(false);
    assert_eq!(style2, style3);
    style3 |= Sty::underline(true);
    assert_eq!(style3, style2);
}

#[test]
fn ops_add() {
    let mut style1 = Sty::secure() + Sty::fg(RED);
    let mut style2 = Sty::bold() + Sty::secure() + Sty::fg(RED);
    let mut style3 = Sty::bold() + Sty::underline(false) + Sty::secure() + Sty::fg(RED);
    let style4 = Sty::bold() + Sty::underline(true) + Sty::secure() + Sty::fg(RED);
    style1 += Sty::bold();
    assert_eq!(style1, style2);
    style2 += Sty::underline(false);
    assert_eq!(style2, style3);
    style2 += Sty::underline(true);
    assert_eq!(style2, style4);
    style3 += Sty::underline(true);
    assert_eq!(style3 + Sty::underline(true), style4);
}

#[test]
fn ops_add_assign() {
    let style1 = Sty::secure() + Sty::fg(RED);
    let style2 = Sty::bold() + Sty::secure() + Sty::fg(RED);
    let style3 = Sty::bold() + Sty::underline(false) + Sty::secure() + Sty::fg(RED);
    let style4 = Sty::bold() + Sty::underline(true) + Sty::secure() + Sty::fg(RED);
    assert_eq!(style1 + Sty::bold(), style2);
    assert_eq!(style2 + Sty::underline(false), style3);
    assert_eq!(style2 + Sty::underline(true), style4);
    assert_eq!(style3 + Sty::underline(true), style4);
}

#[test]
fn ops_sub() {
    let style1 = Sty::secure() | Sty::fg(RED);
    let style2 = Sty::bold() | Sty::secure() | Sty::fg(RED);
    let style3 = Sty::bold() | Sty::underline(false) | Sty::secure() | Sty::fg(RED);
    assert_eq!(style2 - Sty::bold(), style1);
    assert_eq!(style3 - Sty::underline(false), style2);
    assert_eq!(style3 - Sty::underline(true), style3);
}

#[test]
fn ops_sub_assign() {
    let style1 = Sty::secure() | Sty::fg(RED);
    let style2a = Sty::bold() | Sty::secure() | Sty::fg(RED);
    let mut style2b = style2a.clone();
    let style3a = Sty::bold() | Sty::underline(false) | Sty::secure() | Sty::fg(RED);
    let mut style3b = style3a.clone();
    let mut style3c = style3a.clone();
    style2b -= Sty::bold();
    assert_eq!(style2b, style1);
    style3b -= Sty::underline(false);
    assert_eq!(style3b, style2a);
    style3c -= Sty::underline(true);
    assert_eq!(style3c, style3a);
}

#[test]
fn gen_attr_fns() {
    let style1 = Sty::bold() | Sty::underline(true) | Sty::secure() | Sty::fg(RED);
    let mut style2 = Sty::bold().with_underline(true).with_secure().with_fg(RED);
    let mut style3 = style2.with_underline(false);
    let style4 = style2.with_ored_underline(false);

    let mut style1b = style1;
    let mut style2b = style2;

    assert!(style1.has_bold());
    assert!(style1.has_underline());
    assert!(style1.has_exact_underline(true));
    assert!(!style1.has_exact_underline(false));
    assert!(!style1.has_bg());

    assert_eq!(style1, style2);
    assert_ne!(style2, style3);
    assert_eq!(style2, style4);

    style2.unset_underline();
    style3.unset_underline();
    assert_eq!(style2, style3);
    assert_eq!(style1.without_underline().without_fg(), style2.without_fg());

    style1b.unset_secure();
    style2b.unset_secure();
    assert_eq!(style1b, style2b);
    assert_eq!(style1b.without_bold(), style2b.without_bold());
}
