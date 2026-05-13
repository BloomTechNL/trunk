use g_cli::{CoAuthorAliases, RealCoAuthorAliases};
use std::path::PathBuf;
use tempfile::TempDir;

mod common;

pub fn add_alias(
    co_author_aliases_path: &PathBuf,
    alias: &str,
    name: &str,
    email: &str,
) -> anyhow::Result<()> {
    let content = format!("{}:{} <{}>\n", alias, name, email);
    use std::fs::OpenOptions;
    use std::io::Write;
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(co_author_aliases_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
#[test]
fn test_existing_alias() {
    let base_dir = TempDir::new().unwrap();
    let co_author_aliases_path = base_dir.path().join("aliases");
    let co_author_aliases = RealCoAuthorAliases::new(co_author_aliases_path.clone());

    add_alias(
        &co_author_aliases_path,
        "@harry",
        "Harry Bronchitis",
        "harry.bronchitis@example.com",
    )
    .expect("Should succeed");

    let formatted_alias = co_author_aliases
        .format_alias("@harry")
        .expect("Should succeed");

    assert_eq!(
        formatted_alias,
        "Harry Bronchitis <harry.bronchitis@example.com>"
    );
}
