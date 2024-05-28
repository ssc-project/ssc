//! # Svelte Oxide
//!
//! <https://github.com/a-rustacean/svelte-oxide>

pub mod allocator {
    #[doc(inline)]
    pub use oxc_allocator::*;
}

pub mod span {
    #[doc(inline)]
    pub use oxc_span::*;
}

pub mod ast {
    #[doc(inline)]
    pub use svelte_oxide_ast::*;
}

pub mod parser {
    #[doc(inline)]
    pub use svelte_oxide_parser::*;
}

#[cfg(feature = "css")]
pub mod css_ast {
    #[doc(inline)]
    pub use svelte_oxide_css_ast::*;
}

#[cfg(feature = "css")]
pub mod css_parser {
    #[doc(inline)]
    pub use svelte_oxide_css_parser::*;
}

#[cfg(feature = "css")]
pub mod css_codegen {
    #[doc(inline)]
    pub use svelte_oxide_css_codegen::*;
}

#[cfg(feature = "css")]
pub mod css_transformer {
    #[doc(inline)]
    pub use svelte_oxide_css_transformer::*;
}
