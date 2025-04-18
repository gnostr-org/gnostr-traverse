//! Basic cleaner module for Git projects.
use super::Task;
use std::io;

/// Cleaner implementation for Git projects.
pub struct MakeFileTask;
impl Task for MakeFileTask {
    /// Returns the name of this cleaner.
    fn name(&self) -> &str {
        "MakeFile"
    }

    /// Returns the triggers associated with this cleaner.
    fn triggers(&self) -> &[&str] {
        &[
            "Makefile",
            "GNUmakefile",
            "CMakeFiles",
            "CMakeCache",
            ".deps",
            ".libs",
        ]
    }

    /// Cleans the provided directory based on a Git structure.
    fn job(&self, dir: &str) -> io::Result<()> {
        super::cmd(dir, "make", &["clean"])?;
        super::cmd(dir, "rm", &["-rf", "CmakeFiles"])?;
        super::cmd(dir, "rm", &["-rf", "build"])?;
        super::cmd(dir, "rm", &["-rf", ".deps"])?;
        super::cmd(dir, "rm", &["-rf", ".libs"])?;
        super::cmd(dir, "rm", &["-rf", "CmakeCache"])
    }
}
