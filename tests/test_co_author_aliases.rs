use g_cli::{CoAuthorAliases, RealCoAuthorAliases};
use tempfile::TempDir;

mod common;

#[test]
fn test_existing_alias() {
    let base_dir = TempDir::new().unwrap();
    let co_author_aliases_path = base_dir.path().join("aliases");
    let mut co_author_aliases = RealCoAuthorAliases::new(co_author_aliases_path.clone());

    co_author_aliases
        .add_alias("@harry", "Harry Bronchitis", "harry.bronchitis@example.com")
        .expect("Should succeed");

    let formatted_alias = co_author_aliases
        .format_alias("@harry")
        .expect("Should succeed");

    assert_eq!(
        formatted_alias,
        "Harry Bronchitis <harry.bronchitis@example.com>"
    );
}

#[test]
fn test_unknown_alias() {
    let base_dir = TempDir::new().unwrap();
    let co_author_aliases_path = base_dir.path().join("aliases");
    let co_author_aliases = RealCoAuthorAliases::new(co_author_aliases_path.clone());

    let formatted_alias = co_author_aliases.format_alias("@harry");

    assert_eq!(formatted_alias, None,);
}

#[test]
fn test_multiple_aliases() {
    let base_dir = TempDir::new().unwrap();
    let co_author_aliases_path = base_dir.path().join("aliases");
    let mut co_author_aliases = RealCoAuthorAliases::new(co_author_aliases_path.clone());

    co_author_aliases
        .add_alias("@harry", "Harry Bronchitis", "harry.bronchitis@example.com")
        .expect("Should succeed");
    co_author_aliases
        .add_alias("@sally", "Sally Cholera", "sally.cholera@example.com")
        .expect("Should succeed");

    let formatted_alias_1 = co_author_aliases
        .format_alias("@harry")
        .expect("Should succeed");
    let formatted_alias_2 = co_author_aliases
        .format_alias("@sally")
        .expect("Should succeed");

    assert_eq!(
        formatted_alias_1,
        "Harry Bronchitis <harry.bronchitis@example.com>"
    );

    assert_eq!(
        formatted_alias_2,
        "Sally Cholera <sally.cholera@example.com>"
    );
}
