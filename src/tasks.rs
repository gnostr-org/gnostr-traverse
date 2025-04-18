//! Cleaning traits and implementations.
pub mod cargo;
pub mod git;
pub mod gnostr;
pub mod macos;
pub mod makefile;
pub mod node;

pub use cargo::CargoTask;
pub use git::GitTask;
pub use gnostr::GnostrTask;
pub use macos::MacosTask;
pub use makefile::MakeFileTask;
pub use node::NodeTask;

use std::env;
use std::fs;
use std::io::{self, ErrorKind};
use std::process::{Command, Stdio};
use std::str;

/// Trait to represent a cleaning structure.
pub trait Task {
    /// Returns the name of the current cleaner.
    fn name(&self) -> &str;

    /// Cleans a directory assumed to be a relevant directory.
    fn job(&self, dir: &str) -> io::Result<()>;

    /// Returns a set of file names which identify a relevant directory.
    fn triggers(&self) -> &[&str];
}

fn is_program_in_path(program: &str) -> bool {
    if let Ok(path) = env::var("PATH") {
        for p in path.split(":") {
            println!("p={}", p);
            let p_str = format!("{}/{}", p, program);
            if fs::metadata(&p_str).is_ok() {
                println!("p_str={}", p_str);
                return true;
            }
        }
    }
    false
}

/// Executes a command in a directory using provided arguments.
pub fn cmd(dir: &str, cmd: &str, args: &[&str]) -> io::Result<()> {
    let is_command = is_program_in_path(cmd);
    if !is_command {
        let cmd = &"ls";
        let cmd_output = Command::new(cmd)
            .current_dir(dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()?
            .wait_with_output()?;
        if !cmd_output.status.success() {
            println!("Command failed with status: {}", cmd_output.status);
        }
        let cmd_stdout = str::from_utf8(&cmd_output.stdout).expect("");
        println!("ls Stdout: {}", cmd_stdout);
    } else {
        let cmd_output = Command::new(cmd)
            .args(args)
            .current_dir(dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()?
            .wait_with_output()?;
        if !cmd_output.status.success() {
            println!("Command failed with status: {}", cmd_output.status);
        }
        let cmd_stdout = str::from_utf8(&cmd_output.stdout).expect("");
        println!("Cmd Stdout: {}", cmd_stdout);
    }
    Ok(())
}

fn is_directory(filepath: &str) -> io::Result<bool> {
    use std::path::Path;
    let path = Path::new(filepath);
    println!("path_to_string(path)={:?}", path_to_string(path).unwrap());
    match fs::metadata(path) {
        Ok(metadata) => {
            println!("path_to_string(path)={:?}", path_to_string(path));
            Ok(metadata.is_dir())
        }
        Err(e) => Err(e),
    }
}

use std::path::Path;
fn path_to_string(path: &Path) -> Result<String, std::ffi::OsString> {
    path.as_os_str().to_os_string().into_string()
}

pub fn filepath(parent: &str, child: &str) -> io::Result<()> {
    let dir_path = format!("{}/{}", parent, child);
    println!("dir_path={}", dir_path);

    if let Ok(dir) = is_directory(&dir_path.clone()) {
        println!("dir={}", dir);
        match dir {
            true => {
                println!("'{}' is a directory.", dir);
            }
            false => {
                println!("{:?} is NOT a directory!", dir);
            }
        }
    };
    //let dir = !is_directory(&dir_path.clone()).expect("");
    Ok(())
}

/// Purges a location on disk, similar to `rm -rf`.
pub fn del(parent: &str, child: &str) -> io::Result<()> {
    let _ = filepath(parent, child);
    let dir_path = format!("{}/{}", parent, child);
    println!("{}", dir_path);

    // check for errors that we're ok with
    if let Err(err) = fs::remove_dir_all(dir_path) {
        // if already gone, happy days are upon us
        if err.kind() == ErrorKind::NotFound {
            return Ok(());
        }
        // if there's a permission error, we don't care
        if err.kind() == ErrorKind::PermissionDenied {
            return Ok(());
        }
        if err.kind() == ErrorKind::Other {
            let file_path = format!("{}/{}", parent, child);
            println!("{}", file_path);
            // check for errors that we're ok with
            if let Err(err) = fs::remove_file(file_path) {
                // if already gone, happy days are upon us
                if err.kind() == ErrorKind::NotFound {
                    return Ok(());
                }

                // if there's a permission error, we don't care
                if err.kind() == ErrorKind::PermissionDenied {
                    return Ok(());
                }
                if err.kind() == ErrorKind::Other {
                    return Ok(());
                }

                // others, bad!
                // return Err(err);
                println!("{:?}", Some(err));
            }

            return Ok(());
        }

        // others, bad!
        // return Err(err);
        println!("{:?}", Some(err));
    }

    Ok(())
}
