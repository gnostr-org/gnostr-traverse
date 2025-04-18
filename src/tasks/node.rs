//! Basic cleaner module for Node.js projects.
use super::Task;
use std::io;

/// Cleaner implementation for Node.js projects.
pub struct NodeTask;
impl Task for NodeTask {
    /// Returns the name of this cleaner.
    fn name(&self) -> &str {
        "Node.js"
    }

    /// Returns the triggers associated with this cleaner.
    fn triggers(&self) -> &[&str] {
        &["package.json", "yarn.json", ".npm"]
    }

    /// Cleans the provided directory based on a NodeJS structure.
    fn job(&self, dir: &str) -> io::Result<()> {
        //super::del(dir, "node_modules")?;
        //super::del(dir, ".npm")

        super::cmd(dir, "git", &["status"])?;
        super::cmd(dir, "git", &["reflog", "expire", "--all", "--expire=now"])?;
        super::cmd(dir, "git", &["gc", "--prune=now", "--aggressive"])
    }
}
