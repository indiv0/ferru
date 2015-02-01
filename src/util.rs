use error::FerrumResult;

pub fn copy_recursively<F>(source: &Path, dest: &Path, criteria: F) -> FerrumResult<()>
    where F : Fn(&Path) -> bool
{
    use std::old_io;
    use std::old_io::fs;
    use std::old_io::fs::PathExtensions;

    if !source.is_dir() {
        try!(Err(old_io::standard_error(old_io::InvalidInput)))
    }

    let mut contents = try!(fs::walk_dir(source));
    for entry in contents {
        debug!("ENTRY: {}", entry.display());
        if !criteria(&entry) { continue; }

        // TODO: remove this unwrap.
        let new_dest = &dest.join(entry.path_relative_from(source).unwrap());

        if entry.is_dir() {
            try!(fs::mkdir(new_dest, old_io::USER_RWX));
        } else {
            try!(fs::copy(&entry, new_dest));
        }
    }

    Ok(())
}
