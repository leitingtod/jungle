use std::fs::{self, File};
use std::io::Read;
use std::io::Write;
use std::path::Path;

use crate::errors::*;

/// Checks if a file exists, if so, the destination buffer will be filled with
/// its contents.
pub fn load_file_contents<P: AsRef<Path>>(filename: P, dest: &mut Vec<u8>) -> Result<()> {
    let filename = filename.as_ref();

    let mut buffer = Vec::new();
    File::open(filename)?.read_to_end(&mut buffer)?;

    // We needed the buffer so we'd only overwrite the existing content if we
    // could successfully load the file into memory.
    dest.clear();
    dest.append(&mut buffer);

    Ok(())
}

/// Write the given data to a file, creating it first if necessary
pub fn write_file<P: AsRef<Path>>(build_dir: &Path, filename: P, content: &[u8]) -> Result<()> {
    let path = build_dir.join(filename);

    create_file(&path)?.write_all(content).map_err(Into::into)
}

/// This function creates a file and returns it. But before creating the file
/// it checks every directory in the path to see if it exists,
/// and if it does not it will be created.
pub fn create_file(path: &Path) -> Result<File> {
    debug!("Creating {}", path.display());

    // Construct path
    if let Some(p) = path.parent() {
        trace!("Parent directory is: {:?}", p);

        fs::create_dir_all(p)?;
    }

    File::create(path).map_err(Into::into)
}

pub fn remove_dir_content(dir: &Path) -> Result<()> {
    for item in fs::read_dir(dir)? {
        if let Ok(item) = item {
            let item = item.path();
            if item.is_dir() {
                fs::remove_dir_all(item)?;
            } else {
                fs::remove_file(item)?;
            }
        }
    }
    Ok(())
}

pub fn copy_files_except_ext(
    from: &Path,
    to: &Path,
    recursive: bool,
    ext_blacklist: &[&str],
) -> Result<()> {
    debug!(
        "Copying all files from {} to {} (blacklist: {:?})",
        from.display(),
        to.display(),
        ext_blacklist
    );

    // Check that from and to are different
    if from == to {
        return Ok(());
    }

    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let metadata = entry.metadata()?;

        // If the entry is a dir and the recursive option is enabled, call itself
        if metadata.is_dir() && recursive {
            if entry.path() == to.to_path_buf() {
                continue;
            }

            // check if output dir already exists
            if !to.join(entry.file_name()).exists() {
                fs::create_dir(&to.join(entry.file_name()))?;
            }

            copy_files_except_ext(
                &from.join(entry.file_name()),
                &to.join(entry.file_name()),
                true,
                ext_blacklist,
            )?;
        } else if metadata.is_file() {
            // Check if it is in the blacklist
            if let Some(ext) = entry.path().extension() {
                if ext_blacklist.contains(&ext.to_str().unwrap()) {
                    continue;
                }
            }
            debug!(
                "creating path for file: {:?}",
                &to.join(
                    entry
                        .path()
                        .file_name()
                        .expect("a file should have a file name...")
                )
            );

            debug!(
                "Copying {:?} to {:?}",
                entry.path(),
                &to.join(
                    entry
                        .path()
                        .file_name()
                        .expect("a file should have a file name...")
                )
            );
            fs::copy(
                entry.path(),
                &to.join(
                    entry
                        .path()
                        .file_name()
                        .expect("a file should have a file name..."),
                ),
            )?;
        }
    }
    Ok(())
}


/// Prints a "backtrace" of some `Error`.
pub fn log_backtrace(e: &Error) {
    error!("Error: {}", e);

    for cause in e.iter().skip(1) {
        error!("\tCaused By: {}", cause);
    }
}
