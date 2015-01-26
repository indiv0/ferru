use std::collections::HashMap;
use std::io;
use std::io::{fs, File};
use std::io::fs::PathExtensions;

use mustache;

use error::{FerrumError, FerrumResult};
use error::ErrorKind::{InvalidDocumentError, ParserError};
use parser;

#[deriving(PartialEq, Show)]
pub struct Document {
    data: HashMap<String, String>,
    content: String,
}

impl Document {
    pub fn new(header: HashMap<String, String>, content: &str) -> Document {
        Document { data: header, content: content.to_string() }
    }

    pub fn as_html(&self) -> String {
        let template = mustache::compile_str(self.content.as_slice());

        // Write the template to memory, then retrieve it as a string.
        let mut w = io::MemWriter::new();
        template.render(&mut w, &self.data).is_ok();

        w.into_inner().into_ascii().into_string()
    }

    pub fn render_to_file(&self, file_path: &Path, templates: &HashMap<String, String>) -> FerrumResult<()> {
        let template_path = try!(self.template());
        let template = match templates.get(&template_path.to_string()) {
            Some(template) => template,
            None => return Err(FerrumError {
                kind: InvalidDocumentError,
                desc: "Template not found",
                detail: Some(format!("Template path: \"{}\"", template_path))
            })
        };

        fs::mkdir_recursive(&file_path.dir_path(), io::USER_RWX).is_ok();

        let mut file = File::create(file_path);
        let mut data = HashMap::new();

        data.insert("content", self.as_html());

        for (key, value) in self.data.iter() {
            data.insert(key.as_slice(), value.clone());
        }

        let template = mustache::compile_str(template.as_slice());

        template.render(&mut file, &data).is_ok();

        info!("Created {}", file_path.display());
        Ok(())
    }

    fn template(&self) -> FerrumResult<&str> {
        match self.data.get(&"template".to_string()) {
            Some(v) => Ok(v.as_slice()),
            None => Err(FerrumError {
                kind: InvalidDocumentError,
                desc: "Missing template for document",
                detail: None
            })
        }
    }
}

pub fn load_documents_from_disk<F>(documents_path: &Path, criteria: F) -> FerrumResult<HashMap<Path, Document>>
    where F : FnOnce(&Path) -> bool
{
    let mut documents = HashMap::new();

    let mut document_dirs = try!(fs::walk_dir(documents_path));
    for path in document_dirs {
        if !path.is_file() { continue; }
        if !criteria(&path) { continue; }

        // Read the document from the disk.
        let content = try!(File::open(&path).read_to_end());
        let content = String::from_utf8_lossy(content.as_slice());
        let document = match parser::document(content.as_slice()) {
            Ok(document) => document,
            Err(err) => {
                let err = FerrumError {
                    kind: ParserError(err),
                    desc: "Failed to parse a string.",
                    detail: None
                };
                warn!("Failed to read document {}: {}", path.display(), err);
                continue;
            }
        };
        let mut relative_path = path.path_relative_from(documents_path).unwrap();
        relative_path.set_extension("");

        documents.insert(relative_path, document);
    }

    Ok(documents)
}
