
#[cfg(test)]
mod tests {
    use super::*;
    use rusty_sysinfo::model::defaults::Defaults;
    use rusty_sysinfo::model::event::Event;

    #[test]
    fn test_default() {
        let result = Defaults::new();
        assert_eq!(result.frontend_path, format!("{}/frontend-dir", env!("CARGO_MANIFEST_DIR")));
        assert_eq!(result.should_run_terminal, true);
        assert_eq!(result.port, 80);
    }
}
