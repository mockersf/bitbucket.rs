#![deny(
    warnings,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    // missing_docs
)]

//!
//! Bitbucket API Wrapper
//!

mod api;
mod error;
mod internal_api;
mod pagination;
mod repository;
pub mod requests;

pub use api::API;
pub use error::Error;
pub use pagination::{PageIterator, Paginated};

/// Bitbucket object models
pub mod model {
    pub use crate::repository::*;
}
