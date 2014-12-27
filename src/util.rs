use error::FerrumResult;

pub fn copy_recursively(source: &Path, dest: &Path, criteria: |&Path| -> bool) -> FerrumResult<()> {
    use std::io;
    use std::io::fs;
    use std::io::fs::PathExtensions;

    if !source.is_dir() {
        try!(Err(io::standard_error(io::InvalidInput)))
    }

    let contents = try!(fs::readdir(source));
    for entry in contents.iter() {
        if !criteria(entry) { continue; }

        // TODO: remove this unwrap.
        let new_dest = &dest.join(entry.path_relative_from(source).unwrap());

        if entry.is_dir() {
            try!(fs::mkdir(new_dest, io::USER_RWX));
            try!(copy_recursively(entry, new_dest, |p| criteria(p)));
        } else {
            try!(fs::copy(entry, new_dest));
        }
    }

    Ok(())
}
