use screenplay::Ability;

/// Marker ability — signals the actor is allowed to touch the file system.
pub struct UseFileSystem;

impl Ability for UseFileSystem {}
