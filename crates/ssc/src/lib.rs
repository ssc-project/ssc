//! # SSC
//!
//! <https://github.com/ssc-project/ssc>

pub mod allocator {
    #[doc(inline)]
    pub use oxc_allocator::*;
}

pub mod span {
    #[doc(inline)]
    pub use oxc_span::*;
}

pub mod diagnostics {
    #[doc(inline)]
    pub use oxc_diagnostics::*;
}

pub mod ast {
    #[doc(inline)]
    pub use ssc_ast::*;
}

pub mod parser {
    #[doc(inline)]
    pub use ssc_parser::*;
}

pub mod codegen {
    #[doc(inline)]
    pub use ssc_codegen::*;
}

#[cfg(feature = "css")]
pub mod css_ast {
    #[doc(inline)]
    pub use ssc_css_ast::*;
}

#[cfg(feature = "css")]
pub mod css_parser {
    #[doc(inline)]
    pub use ssc_css_parser::*;
}

#[cfg(feature = "css")]
pub mod css_codegen {
    #[doc(inline)]
    pub use ssc_css_codegen::*;
}

#[cfg(feature = "css")]
pub mod css_transformer {
    #[doc(inline)]
    pub use ssc_css_transformer::*;
}

#[cfg(feature = "css")]
pub mod css_analyzer {
    #[doc(inline)]
    pub use ssc_css_analyzer;
}
