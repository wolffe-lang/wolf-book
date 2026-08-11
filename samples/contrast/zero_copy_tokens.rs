//! The Rust half of chapter 7 §7.6's contrast, vendored so it cannot rot.
//!
//! This is the program shape where Rust's lifetime annotations genuinely
//! win: a tokenizer that hands back borrowed slices of an input buffer it
//! does not own. `'a` is what makes it sound — `Token<'a>` cannot outlive
//! the `&'a str` it points into, and the compiler proves it.
//!
//! `cargo xtask contrast` compiles this with warnings denied and runs
//! its assertions, and checks that every ```rust block in the book
//! appears here verbatim. The chapter prints it; CI keeps it true.

pub struct Token<'a> {
    pub text: &'a str,
}

pub fn tokens(input: &str) -> Vec<Token<'_>> {
    input
        .split_whitespace()
        .map(|text| Token { text })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn borrows_the_input() {
        let line = String::from("the wolf runs");
        let ts = tokens(&line);
        assert_eq!(ts[1].text, "wolf");
    }
}
