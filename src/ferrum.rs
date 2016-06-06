use std::fs;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use getopts::Matches;

use document;
use error::FerrumResult;
use template;
use util;

static DEFAULT_SOURCE_PATH: &'static str = "./";
static DEFAULT_DEST_PATH: &'static str = "./_site/";

pub fn build(matches: Matches) -> FerrumResult<()> {
    // Get the source path opt.
    let source = matches.opt_str("s")
        .or(Some(DEFAULT_SOURCE_PATH.to_owned()))
        .as_ref()
        .map(Path::new)
        .map(Path::to_path_buf)
        .unwrap();
    if !source.exists() {
        panic!("Source directory \"{}\" does not exist.", source.display());
    }

    // Get the destination path opt.
    let dest = matches.opt_str("d")
        .or(Some(DEFAULT_DEST_PATH.to_owned()))
        .as_ref()
        .map(Path::new)
        .map(Path::to_path_buf)
        .unwrap();

    debug!("Cleaning destination directory");
    if !dest.exists() {
        println!("Destination directory \"{}\" does not exist, creating.", dest.display());
    } else {
        println!("Cleaning destination directory \"{}\".", dest.display());
        fs::remove_dir_all(&dest).is_ok();
    }
    fs::create_dir(&dest).is_ok();

    // A closure which determines whether or not the provided path is a UNIX
    // hidden file (i.e. if it starts with a `.` character).
    fn is_hidden_file<P>(path: &P) -> bool
        where P: AsRef<Path>,
    {
        let res = util::file_name_from_path(&path).unwrap().starts_with(".");
        debug!("Is path {:?} a hidden file? {:?}", path.as_ref(), res);
        res
    }

    // Load the templates.
    debug!("Loading templates");
    let templates = match template::load_templates_from_disk(&source.join("_templates"), |path| -> bool {
        !is_hidden_file(&path) &&
        path.extension().unwrap() == "tpl"
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
            let res = !is_hidden_file(&path) &&
            path != dest &&
            !path.to_str().unwrap().contains("_posts") &&
            !path.to_str().unwrap().contains("_templates");
            debug!("Is path {:?} a static file? {:?}", path, res);
            res
        };

        match util::copy_recursively(&source, &dest, is_static_file) {
            Err(e) => panic!("{}", e),
            _ => {}
        }
    }

    debug!("Loading documents from disk");
    let documents: HashMap<PathBuf, document::Document> = match document::load_documents_from_disk(&source.join("_posts"), |path| -> bool {
        path.is_file() &&
        !is_hidden_file(&path)
    }) {
        Ok(document) => document,
        Err(err) => panic!("{}", err),
    };

    debug!("Rendering documents");
    for (key, document) in documents.into_iter() {
        let new_dest = dest.join(&key);
        document.render_to_file(&new_dest, &templates).unwrap();
    }

    Ok(())
}
