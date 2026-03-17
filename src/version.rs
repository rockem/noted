use git_version::git_version;

pub const VERSION: &str = git_version!(fallback = "unknown");
