mod common;

macro_rules! create_real_co_author_aliases {
    ($aliases:ident) => {
        use g_cli::{CoAuthorAliases, RealCoAuthorAliases};
        let _macro_temp = tempfile::TempDir::new().unwrap();
        let _macro_path = _macro_temp.path().join("aliases");
        let mut $aliases = RealCoAuthorAliases::new(_macro_path);
    };
}

macro_rules! create_in_memory_co_author_aliases {
    ($aliases:ident) => {
        use crate::common::in_memory_co_author_aliases::InMemoryCoAuthorAliases;
        use g_cli::CoAuthorAliases;
        let mut $aliases = InMemoryCoAuthorAliases::new();
    };
}

macro_rules! aliases_test_suite {
    ($init_macro:ident) => {
        mod $init_macro {
            #[test]
            fn test_existing_alias() {
                $init_macro!(aliases);

                aliases
                    .add_alias("@harry", "Harry Bronchitis", "harry.bronchitis@example.com")
                    .expect("Should succeed");
                let formatted = aliases.format_alias("@harry").expect("Should succeed");

                assert_eq!(formatted, "Harry Bronchitis <harry.bronchitis@example.com>");
            }

            #[test]
            fn test_redefining_alias() {
                $init_macro!(aliases);

                aliases
                    .add_alias("@harry", "Harry typo", "typo.bronchitis@example.com")
                    .expect("Should succeed");
                aliases
                    .add_alias("@harry", "Harry Bronchitis", "harry.bronchitis@example.com")
                    .expect("Should succeed");
                let formatted = aliases.format_alias("@harry").expect("Should succeed");

                assert_eq!(formatted, "Harry Bronchitis <harry.bronchitis@example.com>");
            }

            #[test]
            fn test_unknown_alias() {
                $init_macro!(aliases);

                let formatted = aliases.format_alias("@harry");

                assert_eq!(formatted, None);
            }

            #[test]
            fn test_multiple_aliases() {
                $init_macro!(aliases);

                aliases
                    .add_alias("@harry", "Harry Bronchitis", "harry.bronchitis@example.com")
                    .expect("Should succeed");
                aliases
                    .add_alias("@sally", "Sally Cholera", "sally.cholera@example.com")
                    .expect("Should succeed");

                assert_eq!(
                    aliases.format_alias("@harry").expect("Should succeed"),
                    "Harry Bronchitis <harry.bronchitis@example.com>"
                );
                assert_eq!(
                    aliases.format_alias("@sally").expect("Should succeed"),
                    "Sally Cholera <sally.cholera@example.com>"
                );
            }
        }
    };
}

aliases_test_suite!(create_real_co_author_aliases);
aliases_test_suite!(create_in_memory_co_author_aliases);
