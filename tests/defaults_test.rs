#[cfg(test)]
mod tests {
    use rusty_sysinfo::model::defaults::Defaults;

    #[test]
    fn test_default() {
        let result = Defaults::new();
        assert_eq!(
            result.frontend_path,
            format!("{}/frontend-dir", env!("CARGO_MANIFEST_DIR"))
        );
        assert_eq!(result.port, 80);
    }
}
