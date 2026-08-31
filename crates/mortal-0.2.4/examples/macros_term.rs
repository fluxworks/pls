//! Example of printing to the terminal via macros

#[macro_use] extern crate mortal;

use std::io;

use mortal::{Color, Style, Theme, Terminal};

pub fn main() -> io::Result<()> {
	let term = Terminal::new()?;

    let razzle_dazzle = Color::Red;
    let flooby_doo = Color::Blue;
    let zamabamafoo = Style::BOLD;
    let glitter_exploding = Theme::new(razzle_dazzle, flooby_doo, zamabamafoo);

    term_writeln!(term)?;

    term_writeln!(term,
        [black] "black "
        [blue] "blue "
        [cyan] "cyan "
        [green] "green "
        [magenta] "magenta "
        [red] "red "
        [white] "white "
        [yellow] "yellow"
        [reset])?;

    term_writeln!(term,
        [#black] "black "
        [#blue] "blue "
        [#cyan] "cyan "
        [#green] "green "
        [#magenta] "magenta "
        [#red] "red "
        [#white] "white "
        [#yellow] "yellow"
        [reset])?;

    term_writeln!(term,
        [bold] "bold " [!bold]
        [italic] "italic " [!italic]
        [reverse] "reverse " [!reverse]
        [underline] "underline" [!underline])?;

    term_writeln!(term,
        [fg=razzle_dazzle] "razzle dazzle " [!fg]
        [bg=flooby_doo] "flooby doo " [!bg]
        [style=zamabamafoo] "zamabamafoo!\n" [!style]
        [theme=glitter_exploding] "Like glitter is exploding inside me!" [reset])?;

    term_writeln!(term,
        ("foo {}", 42) " "
        (: "bar") " "
        (? "baz") " "
        "quux")?;

    Ok(())
}
