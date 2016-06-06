use std::fs;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use getopts::Matches;

use document;
use error::{Error, Result};
use template;
use util;

static DEFAULT_SOURCE_PATH: &'static str = "./";
static DEFAULT_DEST_PATH: &'static str = "./_site/";

pub fn build(matches: Matches) -> Result<()> {
    // Get the source path opt.
    let source = matches.opt_str("s")
        .as_ref()
        .map(Path::new)
        .map(Path::to_path_buf)
        .unwrap_or(PathBuf::from(DEFAULT_SOURCE_PATH));
    if !source.exists() {
        return Err(Error::path_not_found(&source))
    }

    // Get the destination path opt.
    let dest = matches.opt_str("d")
        .as_ref()
        .map(Path::new)
        .map(Path::to_path_buf)
        .unwrap_or(PathBuf::from(DEFAULT_DEST_PATH));

    debug!("Cleaning destination directory");
    if !dest.exists() {
        println!("Destination directory \"{}\" does not exist, creating.", dest.display());
    } else {
        println!("Cleaning destination directory \"{}\".", dest.display());
        fs::remove_dir_all(&dest).is_ok();
    }
    fs::create_dir(&dest).is_ok();

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
            !path.to_str().map(|path| path.contains("_posts")).unwrap_or(false) &&
            !path.to_str().map(|path| path.contains("_templates")).unwrap_or(false)
        };

        try!(util::copy_recursively(&source, &dest, is_static_file))
    }

    debug!("Loading documents from disk");
    let documents: HashMap<PathBuf, document::Document> = try!(document::load_documents_from_disk(&source.join("_posts"), |path| {
        !util::is_hidden(&path)
    }));

    debug!("Rendering documents");
    for (key, document) in documents.into_iter() {
        let new_dest = dest.join(&key);
        try!(document.render_to_file(&new_dest, &templates));
    }

    Ok(())
}
