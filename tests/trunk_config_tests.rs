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

            #[test]
            fn test_default_auto_update_period_is_one_week() {
                $init_macro!(config);
                let cfg = config.load();
                assert_eq!(cfg.auto_update_period, 604_800);
            }

            #[test]
            fn test_set_auto_update_period() {
                $init_macro!(config);
                config
                    .set_auto_update_period(3_600)
                    .expect("should succeed");
                let cfg = config.load();
                assert_eq!(cfg.auto_update_period, 3_600);
            }

            #[test]
            fn test_set_auto_update_period_to_zero_disables() {
                $init_macro!(config);
                config.set_auto_update_period(0).expect("should succeed");
                let cfg = config.load();
                assert_eq!(cfg.auto_update_period, 0);
            }
        }
    };
}

trunk_config_test_suite!(create_real_trunk_config);
trunk_config_test_suite!(create_in_memory_trunk_config);

fn non_git_dir() -> tempfile::TempDir {
    tempfile::TempDir::new().expect("should create temp dir")
}

fn git_repo() -> tempfile::TempDir {
    let dir = tempfile::TempDir::new().expect("should create temp dir");
    git2::Repository::init(dir.path()).expect("should init git repo");
    dir
}

macro_rules! create_real_repo_aware_trunk_config {
    ($config:ident, $repo:expr) => {
        use g_cli::RealTrunkConfig;
        let _macro_temp = tempfile::TempDir::new().expect("should create temp dir");
        let _macro_path = _macro_temp.path().join(".config/trunk/trunk.json");
        let $config =
            RepoAwareTrunkConfig::new(RealTrunkConfig::new(_macro_path), $repo.to_path_buf());
    };
}

macro_rules! create_in_memory_repo_aware_trunk_config {
    ($config:ident, $repo:expr) => {
        use crate::common::in_memory_trunk_config::InMemoryTrunkConfig;
        let $config = RepoAwareTrunkConfig::new(InMemoryTrunkConfig::new(), $repo.to_path_buf());
    };
}

macro_rules! repo_aware_trunk_config_test_suite {
    ($init_macro:ident) => {
        mod $init_macro {
            use super::{git_repo, non_git_dir};
            use g_cli::{LocalTrunkConfig, RepoAwareTrunkConfig, TrunkConfig};

            #[test]
            fn test_not_in_a_git_repo_keeps_global_values() {
                let repo = non_git_dir();
                $init_macro!(config, repo.path());
                let cfg = config.load();
                assert!(cfg.co_authors_required);
                assert_eq!(cfg.auto_update_period, 604_800);
            }

            #[test]
            fn test_no_repo_local_file_keeps_global_values() {
                let repo = git_repo();
                $init_macro!(config, repo.path());
                let cfg = config.load();
                assert!(cfg.co_authors_required);
                assert_eq!(cfg.auto_update_period, 604_800);
            }

            #[test]
            fn test_repo_local_file_overrides_co_authors_required_only() {
                let repo = git_repo();
                std::fs::write(
                    repo.path().join(".trunk.json"),
                    r#"{"coAuthorsRequired": false}"#,
                )
                .expect("should write repo-local config");
                $init_macro!(config, repo.path());
                let cfg = config.load();
                assert!(!cfg.co_authors_required);
                assert_eq!(cfg.auto_update_period, 604_800);
            }

            #[test]
            fn test_repo_local_file_overrides_both_fields() {
                let repo = git_repo();
                std::fs::write(
                    repo.path().join(".trunk.json"),
                    r#"{"coAuthorsRequired": false, "autoUpdatePeriod": 3600}"#,
                )
                .expect("should write repo-local config");
                $init_macro!(config, repo.path());
                let cfg = config.load();
                assert!(!cfg.co_authors_required);
                assert_eq!(cfg.auto_update_period, 3_600);
            }

            #[test]
            fn test_repo_local_file_is_discovered_from_nested_subdirectory() {
                let repo = git_repo();
                std::fs::write(
                    repo.path().join(".trunk.json"),
                    r#"{"coAuthorsRequired": false}"#,
                )
                .expect("should write repo-local config");
                let nested = repo.path().join("a/b/c");
                std::fs::create_dir_all(&nested).expect("should create nested dir");
                $init_macro!(config, nested);
                let cfg = config.load();
                assert!(!cfg.co_authors_required);
            }

            #[test]
            fn test_malformed_repo_local_file_falls_back_to_global_values() {
                let repo = git_repo();
                std::fs::write(repo.path().join(".trunk.json"), "not json")
                    .expect("should write repo-local config");
                $init_macro!(config, repo.path());
                let cfg = config.load();
                assert!(cfg.co_authors_required);
                assert_eq!(cfg.auto_update_period, 604_800);
            }

            #[test]
            fn test_setters_delegate_to_the_wrapped_config() {
                let repo = non_git_dir();
                $init_macro!(config, repo.path());
                config
                    .set_co_authors_required(false)
                    .expect("should succeed");
                let cfg = config.load();
                assert!(!cfg.co_authors_required);
            }

            #[test]
            fn test_set_local_co_authors_required_writes_repo_local_file() {
                let repo = git_repo();
                $init_macro!(config, repo.path());
                config
                    .set_local_co_authors_required(false)
                    .expect("should succeed");
                let cfg = config.load();
                assert!(!cfg.co_authors_required);
                assert_eq!(cfg.auto_update_period, 604_800);
            }

            #[test]
            fn test_set_local_auto_update_period_writes_repo_local_file() {
                let repo = git_repo();
                $init_macro!(config, repo.path());
                config
                    .set_local_auto_update_period(3_600)
                    .expect("should succeed");
                let cfg = config.load();
                assert!(cfg.co_authors_required);
                assert_eq!(cfg.auto_update_period, 3_600);
            }

            #[test]
            fn test_set_local_does_not_clobber_a_previously_set_local_field() {
                let repo = git_repo();
                $init_macro!(config, repo.path());
                config
                    .set_local_co_authors_required(false)
                    .expect("should succeed");
                config
                    .set_local_auto_update_period(3_600)
                    .expect("should succeed");
                let cfg = config.load();
                assert!(!cfg.co_authors_required);
                assert_eq!(cfg.auto_update_period, 3_600);
            }

            #[test]
            fn test_local_override_survives_a_global_setter_call() {
                let repo = git_repo();
                $init_macro!(config, repo.path());
                config
                    .set_local_co_authors_required(false)
                    .expect("should succeed");
                config
                    .set_co_authors_required(true)
                    .expect("should succeed");
                let cfg = config.load();
                assert!(
                    !cfg.co_authors_required,
                    "repo-local override should still win"
                );
            }

            #[test]
            fn test_set_local_outside_a_git_repo_fails() {
                let repo = non_git_dir();
                $init_macro!(config, repo.path());
                let result = config.set_local_co_authors_required(false);
                assert!(result.is_err());
            }
        }
    };
}

repo_aware_trunk_config_test_suite!(create_real_repo_aware_trunk_config);
repo_aware_trunk_config_test_suite!(create_in_memory_repo_aware_trunk_config);
