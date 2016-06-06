// Copyright (c) 2016 Nikita Pekin and the ferrum contributors
// See the README.md file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::HashMap;
use std::io::Read;
use std::fs::{self, File};
use std::path::Path;

use error::Result;
use util;

/// A convenient alias type for the type used for storing templates.
pub type TemplateMap = HashMap<String, String>;

/// Recursively traverses the specified directory and loads the all files
/// matching the specified criteria as templates.
pub fn load_templates_from_disk<F>(root_path: &Path, mut criteria: F) -> Result<TemplateMap>
    where F : FnMut(&Path) -> bool
{
    let mut templates = HashMap::new();

    let templates_dir = try!(fs::read_dir(root_path));
    for template_path in templates_dir {
        let template_path = try!(template_path).path();

        if !try!(fs::metadata(&template_path)).is_file() { continue; }
        if !criteria(&template_path) { continue; }

        let template = match File::open(&template_path) {
            Ok(mut template_file) => {
                let mut template = String::new();
                try!(template_file.read_to_string(&mut template));
                template
            },
            Err(e) => {
                warn!("Could not read file {}: {}", template_path.display(), e);
                continue;
            }
        };
        let filename = try!(util::file_name_from_path(&template_path));
        debug!("Loaded template: {}", filename);

        templates.insert(filename.to_owned(), template);
    }

    Ok(templates)
}
