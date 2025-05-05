use console::{Style, Term, style};

pub(crate) fn print_divider_with(fromat_s: &str) {
    let term = Term::stdout();
    let col_sz = term.size().1;
    let sty = Style::from_dotted_str(fromat_s);
    println!("{}", sty.apply_to("-".repeat(col_sz as usize)));
}

#[inline]
pub(crate) fn print_divider() {
    print_divider_with("white.on_black")
}

fn main() {
    println!("This is {} neat", style("quite").cyan());

    print_divider();
    print_divider_with("red.on_black");
}
