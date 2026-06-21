mod common;

macro_rules! create_real_trunk_config {
    ($config:ident) => {
        use g_cli::{RealTrunkConfig, TrunkConfig};
        let _macro_temp = tempfile::TempDir::new().unwrap();
        let _macro_path = _macro_temp.path().join(".config/trunk/trunk.json");
        let $config = RealTrunkConfig::new(_macro_path);
    };
}

macro_rules! create_in_memory_trunk_config {
    ($config:ident) => {
        use crate::common::in_memory_trunk_config::InMemoryTrunkConfig;
        use g_cli::TrunkConfig;
        let $config = InMemoryTrunkConfig::new();
    };
}

macro_rules! trunk_config_test_suite {
    ($init_macro:ident) => {
        mod $init_macro {
            #[test]
            fn test_defaults_to_co_authors_required() {
                $init_macro!(config);
                let cfg = config.load();
                assert!(cfg.co_authors_required);
            }

            #[test]
            fn test_set_co_authors_required_false() {
                $init_macro!(config);
                config
                    .set_co_authors_required(false)
                    .expect("should succeed");
                let cfg = config.load();
                assert!(!cfg.co_authors_required);
            }

            #[test]
            fn test_set_co_authors_required_true() {
                $init_macro!(config);
                config
                    .set_co_authors_required(false)
                    .expect("should succeed");
                config
                    .set_co_authors_required(true)
                    .expect("should succeed");
                let cfg = config.load();
                assert!(cfg.co_authors_required);
            }
        }
    };
}

trunk_config_test_suite!(create_real_trunk_config);
trunk_config_test_suite!(create_in_memory_trunk_config);

#[cfg(test)]
mod real_trunk_config_tests {
    use g_cli::{RealTrunkConfig, TrunkConfig};

    #[test]
    fn test_missing_file_returns_defaults() {
        let temp = tempfile::TempDir::new().unwrap();
        let path = temp.path().join(".config/trunk/trunk.json");
        let config = RealTrunkConfig::new(path);
        let cfg = config.load();
        assert!(cfg.co_authors_required);
    }

    #[test]
    fn test_invalid_json_returns_defaults() {
        let temp = tempfile::TempDir::new().unwrap();
        std::fs::create_dir_all(temp.path().join(".config/trunk")).unwrap();
        let path = temp.path().join(".config/trunk/trunk.json");
        std::fs::write(&path, "not valid json").unwrap();
        let config = RealTrunkConfig::new(path);
        let cfg = config.load();
        assert!(cfg.co_authors_required);
    }

    #[test]
    fn test_valid_json_with_co_authors_required_false() {
        let temp = tempfile::TempDir::new().unwrap();
        std::fs::create_dir_all(temp.path().join(".config/trunk")).unwrap();
        let path = temp.path().join(".config/trunk/trunk.json");
        std::fs::write(&path, r#"{"coAuthorsRequired": false}"#).unwrap();
        let config = RealTrunkConfig::new(path);
        let cfg = config.load();
        assert!(!cfg.co_authors_required);
    }

    #[test]
    fn test_valid_json_with_co_authors_required_true() {
        let temp = tempfile::TempDir::new().unwrap();
        std::fs::create_dir_all(temp.path().join(".config/trunk")).unwrap();
        let path = temp.path().join(".config/trunk/trunk.json");
        std::fs::write(&path, r#"{"coAuthorsRequired": true}"#).unwrap();
        let config = RealTrunkConfig::new(path);
        let cfg = config.load();
        assert!(cfg.co_authors_required);
    }

    #[test]
    fn test_missing_key_returns_default() {
        let temp = tempfile::TempDir::new().unwrap();
        std::fs::create_dir_all(temp.path().join(".config/trunk")).unwrap();
        let path = temp.path().join(".config/trunk/trunk.json");
        std::fs::write(&path, r#"{}"#).unwrap();
        let config = RealTrunkConfig::new(path);
        let cfg = config.load();
        assert!(cfg.co_authors_required);
    }

    #[test]
    fn test_set_co_authors_required_persists_to_file() {
        let temp = tempfile::TempDir::new().unwrap();
        let path = temp.path().join(".config/trunk/trunk.json");
        let config = RealTrunkConfig::new(path.clone());
        config
            .set_co_authors_required(false)
            .expect("should succeed");
        assert!(path.exists());
        let raw = std::fs::read_to_string(&path).unwrap();
        assert!(raw.contains("\"coAuthorsRequired\": false"));
    }
}
