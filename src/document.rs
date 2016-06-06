use std::collections::HashMap;
use std::fs::{self, File};
use std::path::{Path, PathBuf};

use mustache;

use error::{FerrumError, FerrumResult};
use parser;

pub type Header = HashMap<String, String>;

#[derive(PartialEq, Debug)]
pub struct Document {
    data: Header,
    content: String,
}

impl Document {
    pub fn new(header: Header, content: &str) -> Document {
        Document { data: header, content: content.to_string() }
    }

    pub fn as_html(&self) -> String {
        let template = mustache::compile_str(&self.content);

        // Write the template to memory, then retrieve it as a string.
        let mut buf = Vec::<u8>::new();
        template.render(&mut buf, &self.data).is_ok();

        String::from_utf8(buf).unwrap().to_string()
    }

    pub fn render_to_file(&self, file_path: &Path, templates: &HashMap<String, String>) -> FerrumResult<()> {
        let template_path = try!(self.template());
        let template = try!(templates.get(&template_path.to_string())
            .ok_or(FerrumError::missing_template()));

        fs::create_dir_all(&file_path.parent().unwrap()).is_ok();

        let mut file = try!(File::create(file_path));
        let mut data = HashMap::new();

        data.insert("content", self.as_html());

        for (key, value) in self.data.iter() {
            data.insert(&key, value.clone());
        }

        let template = mustache::compile_str(&template);

        template.render(&mut file, &data).is_ok();

        info!("Created {}", file_path.display());
        Ok(())
    }

    fn template(&self) -> FerrumResult<&String> {
        self.data.get(&"template".to_string())
            .ok_or(FerrumError::missing_template_field())
    }
}

pub fn load_documents_from_disk<F>(documents_path: &Path, mut criteria: F) -> FerrumResult<HashMap<PathBuf, Document>>
    where F : FnMut(&Path) -> bool
{
    use util;

    let mut documents = HashMap::new();

    try!(util::walk_dir(
        documents_path,
        &mut |path| {
            if !criteria(&path) {
                return Ok(());
            }

            // Read the document from the disk.
            let content = {
                use std::io::Read;

                let mut buf = String::new();
                let mut file = try!(File::open(&path));
                try!(file.read_to_string(&mut buf));
                buf
            };
            let document = try!(parser::parse_document(&content));
            let relative_path = try!(path.strip_prefix(documents_path));

            documents.insert(relative_path.with_extension("").to_path_buf(), document);

            Ok(())
        },
    ));

    Ok(documents)
}
