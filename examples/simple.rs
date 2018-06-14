extern crate term_string;

use term_string::TermString as Str;
use term_string::TermStyle as Sty;
use term_string::color::{GREEN, RED};

fn main() {
    let mut style = Sty::bold() + Sty::fg(GREEN) + Sty::bg(RED);
    let t_str = Str::new(style, "BOLD FG=GREEN BG=RED");
    t_str.println();

    style -= Sty::bold();
    style += Sty::underline(true);
    let t_str = Str::new(style, "UNDERLINE FG=GREEN BG=RED");
    t_str.println();
}
