mod column;
mod dataframe;
mod error;
mod schema;

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
