//! Cleaning of unnecessary files in development directories.
#![doc(html_root_url = "https://docs.rs/detox/0.1.2")]
mod options;
mod tasks;

use walkdir::WalkDir;

use crate::options::Options;

use std::env;
use std::error::Error;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    // parse in our options from the command line args
    let options = Options::from(env::args_os());

    // iterate each provided location
    for location in &options.locations {
        // grab the size of the location before we start
        let start = get_size(location);

        // iterate all file entries that we come across in the recursive walk
        for entry in WalkDir::new(location).into_iter().filter_map(Result::ok) {
            // grab the full path
            let path = entry.path();
            println!("{}", path.display());

            // fetch the file name
            let segment = path
                .file_name()
                .unwrap()
                .to_str()
                .expect("a segment should exist");

            // walk through all cleaners
            for task in &options.tasks {
                // skip if the cleaner doesn't care
                if !task.triggers().contains(&segment) {
                    continue;
                }

                // grab the dir
                let dir = path
                    .parent()
                    .unwrap()
                    .to_str()
                    .expect("dir should be a str");

                // clean the directory
                print!("{} ", task.name());
                task.job(dir)?;
            }
        }

        // fetch the size of the location when done
        let end = get_size(location);

        if start >= end {
            // output the stats
            println!(
                "Reduced {} from {} to {} ({:.2}%)",
                location.display(),
                start,
                end,
                ((start - end) as f64 / start as f64) * 100.0
            )
        } else if start == end {
            println!("No reduction in size!")
        } else {
        }
    }

    // done!
    Ok(())
}

/// Determines the size of a directory on the filesystem.
fn get_size(path: &Path) -> u64 {
    WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|node| node.file_type().is_file())
        .filter_map(|file| file.metadata().ok())
        .map(|meta| meta.len())
        .sum()
}
