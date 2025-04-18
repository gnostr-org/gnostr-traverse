//! Basic cleaner module for Cargo projects.
use super::Task;
use std::io;

/// Cleaner implementation for Gnostr Artifacts
pub struct GnostrTask;
impl Task for GnostrTask {
    /// Returns the name of this cleaner.
    fn name(&self) -> &str {
        "Gnostr"
    }

    /// Returns the triggers associated with this cleaner.
    fn triggers(&self) -> &[&str] {
        &[".gnostr"]
    }

    /// cleaner the provided directory based on a Cargo structure.
    fn job(&self, dir: &str) -> io::Result<()> {
        super::cmd(dir, "cargo", &["clean"])?;
        super::del(dir, "node_modules")
    }
}
