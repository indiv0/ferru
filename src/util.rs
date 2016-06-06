use std::fs;
use std::io::{
    Error as IoError,
    ErrorKind,
};
use std::path::Path;

use error::{FerrumError, FerrumResult};

pub fn copy_recursively<F>(source: &Path, dest: &Path, criteria: F) -> FerrumResult<()>
    where F : Fn(&Path) -> bool
{
    if !source.is_dir() {
        debug!("Source path {:?} is not a directory.", source);
        try!(Err(IoError::new(ErrorKind::InvalidInput, "Invalid input")))
    }

    debug!("Copying directory {:?} to {:?} recursively.", source, dest);
    try!(walk_dir(
        source,
        &mut |entry| {
            debug!("Entry: {:?}", entry);
            if !criteria(entry) {
                return Ok(());
            }

            debug!("Stripped path: {:?}", entry.strip_prefix(source));
            let new_dest = &dest.join(try!(entry.strip_prefix(source)));

            if entry.is_dir() {
                debug!("Creating directory: {:?}", new_dest);
                try!(fs::create_dir(new_dest));
            } else {
                debug!("Copying file: {:?} to directory: {:?}", entry, new_dest);
                try!(fs::copy(&entry, new_dest));
            }

            Ok(())
        },
    ));

    Ok(())
}

pub fn walk_dir<F, P>(path: P, action: &mut F) -> FerrumResult<()>
    where F: FnMut(&Path) -> FerrumResult<()>,
          P: AsRef<Path>,
{
    debug!("Walking directory for path: {:?}", path.as_ref());
    for entry in try!(fs::read_dir(path)) {
        let entry = try!(entry);

        try!(action(&entry.path()));

        if try!(fs::metadata(&entry.path())).is_dir() {
            try!(walk_dir(
                &entry.path(),
                action,
            ));
        }
    }

    Ok(())
}

pub fn file_name_from_path<P>(path: &P) -> FerrumResult<&str>
    where P: AsRef<Path>,
{
    try!(path.as_ref().file_name().ok_or(FerrumError::MissingFileName))
        .to_str().ok_or(FerrumError::InvalidUtf8)
}

/// Determines whether or not the provided path item is hidden (i.e. if it
/// starts with a `.` character).
pub fn is_hidden<P>(path: &P) -> bool
    where P: AsRef<Path>,
{
    file_name_from_path(&path)
        .map(|file_name| file_name.starts_with("."))
        .unwrap_or(false)
}
