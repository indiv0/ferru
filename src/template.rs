use std::collections::HashMap;
use std::io::{fs, File};
use std::io::fs::PathExtensions;

use mustache;
use mustache::Template;

use error::FerrumResult;

pub fn load_templates_from_disk(root_path: &Path, criteria: |&Path| -> bool) -> FerrumResult<HashMap<String, Template>> {
    let mut templates = HashMap::new();

    let templates_dir = try!(fs::readdir(&root_path.join("_templates")));
    for template_path in templates_dir.iter() {
        if !template_path.is_file() { continue; }
        if !criteria(template_path) { continue; }

        let raw_template = match File::open(template_path).read_to_string() {
            Ok(v) => v,
            Err(e) => {
                warn!("Could not read file {}: {}", template_path.display(), e);
                continue;
            }
        };
        let template = mustache::compile_str(raw_template.as_slice());

        // Remove extension from template_path.
        let mut template_path = template_path.clone();
        template_path.set_extension("");
        let filename_str = template_path.filename_str().unwrap().to_string();

        templates.insert(filename_str, template);
    }

    Ok(templates)
}
