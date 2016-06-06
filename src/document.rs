use std::collections::HashMap;
use std::fs::{self, File};
use std::path::{Path, PathBuf};

use mustache;

use error::{Error, Result};
use parser;
use template::TemplateMap;

pub type Header = HashMap<String, String>;

#[derive(PartialEq, Debug)]
pub struct Document {
    data: Header,
    content: String,
}

impl Document {
    pub fn new(header: Header, content: &str) -> Document {
        Document { data: header, content: content.to_owned() }
    }

    pub fn as_html(&self) -> Result<String> {
        let template = mustache::compile_str(&self.content);

        // Write the template to memory, then retrieve it as a string.
        let mut buf = Vec::<u8>::new();
        try!(template.render(&mut buf, &self.data));

        String::from_utf8(buf).map_err(Error::from)
    }

    pub fn render_to_file(&self, file_path: &Path, templates: &TemplateMap) -> Result<()> {
        let template_path = try!(self.template());
        let template = try!(templates.get(&template_path.to_owned())
            .ok_or(Error::TemplateNotFound));

        let parent_path = file_path.parent().ok_or(Error::missing_parent_path(&file_path));
        try!(fs::create_dir_all(&try!(parent_path)));

        let mut file = try!(File::create(file_path));
        let mut data = HashMap::new();

        data.insert("content", try!(self.as_html()));

        for (key, value) in self.data.iter() {
            data.insert(&key, value.clone());
        }

        let template = mustache::compile_str(&template);

        try!(template.render(&mut file, &data));

        info!("Created {}", file_path.display());
        Ok(())
    }

    fn template(&self) -> Result<&String> {
        self.data.get(&"template".to_owned())
            .ok_or(Error::MissingTemplateField)
    }
}

pub fn load_documents_from_disk<F>(documents_path: &Path, mut criteria: F) -> Result<HashMap<PathBuf, Document>>
    where F : FnMut(&Path) -> bool
{
    use util;

    let mut documents = HashMap::new();

    try!(util::walk_dir(
        documents_path,
        &mut |path| {
            if !criteria(&path) || !path.is_file() {
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
