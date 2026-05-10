//! `minidf` is a small, educational dataframe library.
//!
//! The crate is intended as a learning project for building a typed,
//! columnar dataframe from first principles in Rust.
//!
//! The current design prioritises:
//!
//! - explicit schemas
//! - typed nullable columns
//! - clear construction invariants
//! - immutable dataframe operations
//! - simple, testable semantics
//!
//! Performance and completeness are not major considerations.

mod column;
mod dataframe;
mod error;
mod schema;
mod value;

pub use column::{Column, DataType};
pub use dataframe::DataFrame;
pub use error::{MiniDfError, Result};
pub use schema::{Field, Schema};

#[cfg(test)]
mod tests {
    #[test]
    fn crate_exposes_package_metadata() {
        let name = env!("CARGO_PKG_NAME");
        let version = env!("CARGO_PKG_VERSION");

        assert_eq!(name, "minidf");
        assert!(!version.is_empty());
    }
}
