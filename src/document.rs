use std::collections::HashMap;
use std::old_io;
use std::old_io::{fs, File};
use std::old_io::fs::PathExtensions;
use std::path::BytesContainer;

use mustache;

use error::{FerrumError, FerrumResult};
use parser;

#[derive(PartialEq, Show)]
pub struct Document {
    data: HashMap<String, String>,
    content: String,
}

impl Document {
    pub fn new(header: HashMap<String, String>, content: &str) -> Document {
        Document { data: header, content: content.to_string() }
    }

    pub fn as_html(&self) -> String {
        let template = mustache::compile_str(&*self.content);

        // Write the template to memory, then retrieve it as a string.
        let mut w = Vec::<u8>::new();
        //let mut w = old_io::MemWriter::new();
        template.render(&mut w, &self.data).is_ok();

        w.container_as_str().unwrap().to_string()
    }

    pub fn render_to_file(&self, file_path: &Path, templates: &HashMap<String, String>) -> FerrumResult<()> {
        let template_path = try!(self.template());
        let template = match templates.get(&template_path.to_string()) {
            Some(template) => template,
            None => return Err(FerrumError::InvalidDocumentError("Template not found".to_string()))
        };

        fs::mkdir_recursive(&file_path.dir_path(), old_io::USER_RWX).is_ok();

        let mut file = File::create(file_path);
        let mut data = HashMap::new();

        data.insert("content", self.as_html());

        for (key, value) in self.data.iter() {
            data.insert(&key[], value.clone());
        }

        let template = mustache::compile_str(&template[]);

        template.render(&mut file, &data).is_ok();

        info!("Created {}", file_path.display());
        Ok(())
    }

    fn template(&self) -> FerrumResult<&str> {
        match self.data.get(&"template".to_string()) {
            Some(v) => Ok(&v[]),
            None => Err(FerrumError::InvalidDocumentError("Missing template".to_string()))
        }
    }
}

pub fn load_documents_from_disk<F>(documents_path: &Path, mut criteria: F) -> FerrumResult<HashMap<Path, Document>>
    where F : FnMut(&Path) -> bool
{
    let mut documents = HashMap::new();

    let mut document_dirs = try!(fs::walk_dir(documents_path));
    for path in document_dirs {
        if !path.is_file() { continue; }
        if !criteria(&path) { continue; }

        // Read the document from the disk.
        let content = try!(File::open(&path).read_to_end());
        let content = String::from_utf8_lossy(&*content);
        let document = match parser::document(&*content) {
            Ok(document) => document,
            Err(err) => {
                warn!("Failed to read document {}: {}", path.display(), FerrumError::ParserError(err));
                continue;
            }
        };
        let mut relative_path = path.path_relative_from(documents_path).unwrap();
        relative_path.set_extension("");

        documents.insert(relative_path, document);
    }

    Ok(documents)
}
