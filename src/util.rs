use mustache::Data;

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

pub fn copy_data<'a>(data: &Data<'a>) -> Data<'a> {
    use std::collections::HashMap;

    use mustache::Data::{Map, StrVal, VecVal};

    match data {
        &StrVal(ref v) => StrVal(v.clone()),
        &VecVal(ref v) => VecVal({
            let mut new = Vec::new();
            for item in v.iter() {
                new.push(copy_data(item));
            }
            new
        }),
        &Map(ref v) => Map({
            let mut new = HashMap::new();
            for (key, value) in v.iter() {
                new.insert(key.clone(), copy_data(value));
            }
            new
        }),
        _ => panic!("Unexpected data: {}", data)
    }
}
