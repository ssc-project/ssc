use lazy_static::lazy_static;
use oxc_allocator::{Allocator, Vec};
use oxc_span::Atom;
use regex::Regex;

lazy_static! {
    pub static ref REGEX_SVELTE_IGNORE: Regex =
        Regex::new(r"(?m)^\s*svelte-ignore\s+([\s\S]+)\s*$").unwrap();
}

pub fn extract_svelte_ignore<'a>(
    allocator: &'a Allocator,
    data: &'a str,
) -> Vec<'a, Atom<'a>> {
    let mut ignores = Vec::new_in(allocator);

    if let Some(captures) = REGEX_SVELTE_IGNORE.captures(data) {
        let ignores_mat = captures.get(1).unwrap();
        for ignore in ignores_mat.as_str().split(&[' ', '\n', '\t', '\r']) {
            let ignore = ignore.trim();
            if ignore.is_empty() {
                continue;
            }
            ignores.push(Atom::from(ignore));
        }
    }

    ignores
}
