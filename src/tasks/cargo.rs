use super::Task;
use std::io;

pub struct CargoTask;
impl Task for CargoTask {
    fn name(&self) -> &str {
        "Cargo"
    }

    fn triggers(&self) -> &[&str] {
        &["Cargo.toml", "vendor", ".cargo"]
    }

    fn job(&self, dir: &str) -> io::Result<()> {
        super::cmd(dir, "cargo", &["build"])?;
        super::cmd(dir, "cargo", &["+nightly", "fmt"])?;
        super::cmd(dir, "cargo", &["+nightly", "build"])?;
        //super::cmd(dir, "cargo", &["clean"])?;

        //super::del(dir, "target")?;
        //super::del(dir, "vendor") //?;
        //super::del(dir, ".cargo/registry")

        super::cmd(dir, "git", &["diff", "HEAD"])
    }
}
