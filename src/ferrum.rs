use std::old_io as io;
use std::old_io::fs;
use std::old_io::fs::PathExtensions;
use std::collections::HashMap;

use getopts::Matches;

use document;
use template;
use util;

static DEFAULT_SOURCE_PATH: &'static str = "./";
static DEFAULT_DEST_PATH: &'static str = "./_site/";

pub fn build(matches: Matches) {
    // Get the source path opt.
    let source = match matches.opt_str("s") {
        Some(v) => Path::new(v),
        None => Path::new(DEFAULT_SOURCE_PATH)
    };
    if !source.exists() {
        panic!("Source directory \"{}\" does not exist.", source.display());
    }

    // Get the destination path opt.
    let dest = match matches.opt_str("d") {
        Some(v) => Path::new(v),
        None => Path::new(DEFAULT_DEST_PATH)
    };

    debug!("Cleaning destination directory");
    if !dest.exists() {
        println!("Destination directory \"{}\" does not exist, creating.", dest.display());
    } else {
        println!("Cleaning destination directory \"{}\".", dest.display());
        fs::rmdir_recursive(&dest).is_ok();
    }
    fs::mkdir(&dest, io::USER_RWX).is_ok();

    // Load the templates.
    debug!("Loading templates");
    let templates = match template::load_templates_from_disk(&source.join("_templates"), |p| -> bool {
        !p.filename_str().unwrap().starts_with(".") &&
        p.extension_str().unwrap() == "tpl"
    }) {
        Ok(v) => v,
        Err(e) => {
            println!("Failed to read templates: {}", e);
            return;
        }
    };

    // Copy all non-template and non-document content.
    debug!("Copying static files");
    if source != dest {
        match util::copy_recursively(&source, &dest, |p| -> bool {
            !p.filename_str().unwrap().starts_with(".") &&
            p != &dest &&
            !p.as_str().unwrap().contains("_posts") &&
            !p.as_str().unwrap().contains("_templates")
        }) {
            Err(e) => panic!("{}", e),
            _ => {}
        }
    }

    debug!("Loading documents from disk");
    let documents: HashMap<Path, document::Document>  = document::load_documents_from_disk(&source.join("_posts"), |p| -> bool {
        !p.filename_str().unwrap().starts_with(".")
    }).unwrap();

    debug!("Rendering documents");
    for (key, document) in documents.into_iter() {
        let new_dest = dest.join(&key);
        document.render_to_file(&new_dest, &templates).unwrap();
    }
}
