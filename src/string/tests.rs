/*
    This file is a part of term-string.

    Copyright (C) 2018 Mohammad AlSaleh <CE.Mohammad.AlSaleh at gmail.com>
    https://github.com/rust-alt/term-string

    This Source Code Form is subject to the terms of the Mozilla Public
    License, v. 2.0. If a copy of the MPL was not distributed with this
    file, You can obtain one at <http://mozilla.org/MPL/2.0/>.
*/

use string::TermString as Str;
use style::TermStyle as Sty;

// Essentials

#[test]
fn new() {
    let text = "Hello World!";
    let style = Sty::bold();
    let t_str = Str::new(style, text);
    assert!(t_str.elements.len() == 1);
    assert!(t_str.elements[0].text == text);
    assert!(t_str.elements[0].style == style);
}

#[test]
fn len() {
    let text1 = "Hello ";
    let text2 = "World!";
    let style1 = Sty::bold();
    let style2 = Sty::underline(true);
    let t_str1 = Str::new(style1, text1);
    let t_str2 = Str::new(style2, text2);

    let text = String::from(text1) + text2;
    let t_str = t_str1 + t_str2;

    assert!(t_str.elements.len() == 2);
    assert!(t_str.elements[0].style == style1);
    assert!(t_str.elements[1].style == style2);
    assert_eq!(t_str.len(), text.len());
}

#[test]
fn is_empty() {
    let mut ts = Str::new(Sty::bold(), "");
    ts += Str::new(Sty::underline(true), "");
    assert!(ts.is_empty());
}

#[test]
fn as_string() {
    let text1 = "Hello ";
    let text2 = "World!";
    let style = Sty::bold();
    let t_str = Str::new(style, text1) + text2;

    let text = String::from(text1) + text2;
    assert!(t_str.elements.len() == 1);
    assert_eq!(t_str.as_string(), text);
}

#[test]
fn append_str() {
    let text1 = "Hello ";
    let text2 = "World!";
    let style = Sty::bold();

    let mut t_str = Str::default();
    t_str.append_str("");
    t_str += Str::new(style, text1);
    t_str.append_str(text2);

    let text = String::from(text1) + text2;
    assert!(t_str.elements.len() == 1);
    assert_eq!(t_str.as_string(), text);
}

#[test]
fn with_appended_str() {
    let text1 = "Hello ";
    let text2 = "World!";
    let style = Sty::bold();

    let t_str = Str::new(style, text1);

    let text = String::from(text1) + text2;
    assert_eq!(t_str.with_appended_str(text2).as_string(), text);
}

#[test]
fn append_term_str() {
    let mut t_str1 = Str::default();
    let t_str2 = Str::new(Sty::default(), "  ");
    t_str1.append_term_str(" ");
    t_str1.append_term_str(" ");
    assert_eq!(t_str1.elements, t_str2.elements);
}

#[test]
fn with_appended_term_str() {
    let t_str1 = Str::default();
    let t_str2 = Str::new(Sty::default(), " ");
    assert_eq!(t_str1.with_appended_term_str(" ").elements, t_str2.elements);
}

// Style

#[test]
fn set_style() {
    let text1 = "Hello ";
    let text2 = "World!";
    let style1 = Sty::underline(true);
    let style2 = Sty::underline(false);
    let t_str1 = Str::new(style1, text1);
    let t_str2 = Str::new(style2, text2);

    let text = String::from(text1) + text2;
    let mut t_str = t_str1 + t_str2;
    t_str.set_style(Sty::bold());

    assert!(t_str.elements.len() == 2);
    assert!(t_str.elements[0].style == Sty::bold());
    assert!(t_str.elements[1].style == Sty::bold());
    assert_eq!(t_str.len(), text.len());
}

#[test]
fn with_new_style() {
    let text1 = "Hello ";
    let text2 = "World!";
    let style1 = Sty::underline(true);
    let style2 = Sty::underline(false);
    let t_str1 = Str::new(style1, text1);
    let t_str2 = Str::new(style2, text2);

    let text = String::from(text1) + text2;
    let t_str = (t_str1 + t_str2).with_set_style(Sty::bold());

    assert!(t_str.elements.len() == 2);
    assert!(t_str.elements[0].style == Sty::bold());
    assert!(t_str.elements[1].style == Sty::bold());
    assert_eq!(t_str.len(), text.len());
}

#[test]
fn reset_style() {}

#[test]
fn with_reset_style() {}

#[test]
fn or_style() {}

#[test]
fn add_style() {}

#[test]
fn with_or_style() {}

#[test]
fn with_style() {}
