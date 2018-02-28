// Copyright (c) 2016, 2018 Nikita Pekin and the ferru contributors
// See the README.md file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fs;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use config::Config;
use document;
use error::{Error, Result};
use template;
use util;

/// Reads all relevant files in the specified source directory, uses them to
/// generate a static website, and stores the resulting files in the specified
/// destination directory.
pub fn build(config: &Config) -> Result<()> {
    const DEFAULT_SOURCE_PATH: &'static str = "./";
    const DEFAULT_DEST_PATH: &'static str = "./_site/";

    // Get the source path opt.
    let source = config.source_directory.as_ref()
        .map(Path::new)
        .map_or_else(|| PathBuf::from(DEFAULT_SOURCE_PATH), Path::to_path_buf);
    if !source.exists() {
        return Err(Error::path_not_found(&source))
    }

    // Get the destination path opt.
    let dest = config.dest_directory.as_ref()
        .map(Path::new)
        .map_or_else(|| PathBuf::from(DEFAULT_DEST_PATH), Path::to_path_buf);

    debug!("Cleaning destination directory");
    if !dest.exists() {
        println!("Destination directory \"{}\" does not exist, creating.", dest.display());
        fs::create_dir(&dest)?;
    } else {
        println!("Cleaning destination directory \"{}\".", dest.display());
        remove_dir_contents(&dest)?;
    }

    // Load the templates.
    debug!("Loading templates");
    let templates = match template::load_templates_from_disk(&source.join("_templates"), |path| {
        !util::is_hidden(&path) &&
        path.extension().and_then(OsStr::to_str) == Some("tpl")
    }) {
        Ok(v) => v,
        Err(e) => {
            println!("Failed to read templates: {}", e);
            return Err(e);
        }
    };

    // Copy all non-template and non-document content.
    debug!("Copying static files");
    if source != dest {
        let is_static_file = |path: &Path| {
            !util::is_hidden(&path) &&
            path != dest &&
            !path.to_str().map_or(false, |path| path.contains("_posts")) &&
            !path.to_str().map_or(false, |path| path.contains("_templates"))
        };

        try!(util::copy_recursively(&source, &dest, is_static_file))
    }

    debug!("Loading documents from disk");
    let documents: HashMap<PathBuf, document::Document> = try!(document::load_documents_from_disk(&source.join("_posts"), |path| {
        !util::is_hidden(&path)
    }));

    debug!("Rendering documents");
    // Render non-static files to a www/ directory.
    let dest = dest.join("www");
    for (key, document) in documents.into_iter() {
        let new_dest = dest.join(&key);
        try!(document.render_to_file(&new_dest, &templates));
    }

    Ok(())
}

fn remove_dir_contents(path: &Path) -> Result<()> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                fs::remove_dir_all(path)?;
            } else {
                fs::remove_file(path)?;
            }
        }
    } else {
        warn!("Attempted to remove dir contents for a file path");
    }

    Ok(())
}
