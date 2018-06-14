extern crate term_string;

use term_string::TermString as Str;
use term_string::TermStyle as Sty;
use term_string::color::{RED, WHITE, MAGENTA, BLUE};

fn main() {
    let style1 = Sty::fg(WHITE) | Sty::bold() | Sty::bg(RED);
    let style2 = Sty::fg(WHITE) | Sty::underline(true) | Sty::bg(BLUE);
    Str::new(style1, " FG WHITE | BOLD | BG RED ").println();
    Str::new(style2, " FG WHITE | UNDERLINE | BG BLUE ").println();
    // bg already set, ORing won't override
    Str::new(style2 | Sty::bg(MAGENTA), " SAME ").println();
    // bg already set, ANDing does override
    Str::new(style2 + Sty::bg(MAGENTA), " MAGENTA-BG ").println();
    Str::new(style2 | Sty::bold(), " BOLD ").println();
    Str::new(style2 | Sty::reverse(), " REVERSE ").println();
    Str::new(style2 | Sty::italic(true), " ITALIC ").println();
    Str::new(style2 | Sty::secure(), " SECURE ").println();
    Str::new(style2 | Sty::standout(true), " STANDOUT ").println();

    let mut test_append = Str::new(style1, "Append");
    test_append += " >> same ";
    test_append += Str::new(style2, " > style2 ");
    test_append += " >> same ";
    test_append += Str::from(" > not_formatted ");
    test_append += " >> same ";
    test_append.println();
}
