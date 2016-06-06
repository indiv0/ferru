use std::fmt;
use std::fs;
use std::path::Path;

use error::{Error, Result};

pub fn copy_recursively<F, P>(source: &P, dest: &P, criteria: F) -> Result<()>
    where F: Fn(&Path) -> bool,
          P: AsRef<Path> + fmt::Debug,
{
    if !source.as_ref().is_dir() {
        return Err(Error::path_is_not_a_directory(source))
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
            let new_dest = &dest.as_ref().join(try!(entry.strip_prefix(source)));

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

pub fn walk_dir<F, P>(path: P, action: &mut F) -> Result<()>
    where F: FnMut(&Path) -> Result<()>,
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

pub fn file_name_from_path<P>(path: &P) -> Result<&str>
    where P: AsRef<Path>,
{
    try!(path.as_ref().file_name().ok_or(Error::MissingFileName))
        .to_str().ok_or(Error::InvalidUtf8)
}

/// Determines whether or not the provided path item is hidden (i.e. if it
/// starts with a `.` character).
pub fn is_hidden<P>(path: &P) -> bool
    where P: AsRef<Path>,
{
    file_name_from_path(&path)
        .map(|file_name| file_name.starts_with('.'))
        .unwrap_or(false)
}
