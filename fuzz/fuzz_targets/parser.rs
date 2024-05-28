#![no_main]

use oxc_allocator::Allocator;
use svelte_oxide_parser::Parser;

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        if s.chars().all(|s| !s.is_control()) {
            let allocator = Allocator::default();
            let _ = Parser::new(&allocator, &s).parse();
        }
    }
});
