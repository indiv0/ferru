use std::collections::HashMap;
use std::io::{fs, File};
use std::io::fs::PathExtensions;

use error::FerrumResult;

pub fn load_templates_from_disk(root_path: &Path, criteria: |&Path| -> bool) -> FerrumResult<HashMap<String, String>> {
    let mut templates = HashMap::new();

    let templates_dir = try!(fs::readdir(root_path));
    for template_path in templates_dir.iter() {
        if !template_path.is_file() { continue; }
        if !criteria(template_path) { continue; }

        let template = match File::open(template_path).read_to_string() {
            Ok(v) => v,
            Err(e) => {
                warn!("Could not read file {}: {}", template_path.display(), e);
                continue;
            }
        };
        let filename = template_path.filename_str().unwrap().to_string();
        debug!("Loaded template: {}", filename);

        templates.insert(filename, template);
    }

    Ok(templates)
}
