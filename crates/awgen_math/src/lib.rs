//! A collection of simple math utilities used by Awgen.

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(rustdoc::invalid_codeblock_attributes)]
#![warn(rustdoc::invalid_html_tags)]


pub mod iterators;
pub mod region;


/// A re-export of all components and systems defined within this crate.
pub mod prelude {
    pub use super::iterators::*;
    pub use super::region::*;
}
