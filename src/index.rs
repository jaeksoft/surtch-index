use std::collections::HashMap;
use reader::FieldReader;
use document::Document;
use std::path::{Path, PathBuf};
use std::fs;
use std::fs::DirEntry;
use writer::SegmentWriter;
use fst::Result;
use query::Query;
use rayon::prelude::*;

pub struct Index {
    /// The path of the index directory
    pub path: PathBuf,
    /// The available catalogs
    fields: HashMap<String, FieldReader>,
}

impl Index {
    /// Open an existing index, or create a new one.
    pub fn open(index_path: &Path) -> Result<Index> {
        if !index_path.exists() {
            fs::create_dir(index_path)?
        }
        // Read the fields
        let fields = HashMap::new();
        let mut index = Index { path: index_path.to_path_buf(), fields };
        index.reload()?;
        return Ok(index);
    }

    fn reload(&mut self) -> Result<()> {
        // List the field directories
        for entry in fs::read_dir(self.path.as_path())? {
            let dir_entry = entry?;
            if dir_entry.file_type()?.is_dir() {
                let file_name = dir_entry.file_name().into_string().unwrap();
                let field_reader = self.fields.entry(file_name).or_insert_with(|| FieldReader::open(dir_entry.path()).unwrap());
            }
        }
        // Concurrent reload
        self.fields.par_iter_mut().for_each(|(field_name, field_reader)| field_reader.reload().unwrap());
        return Ok({});
    }

    ///
    /// Create a new segment which will contains all the documents
    ///
    pub fn put(&mut self, documents: &Vec<Document>) -> Result<()> {
        SegmentWriter::index(self.path.to_str().unwrap(), documents)?;
        self.reload()?;
        return Ok({});
    }

    pub fn find(&self, query: &Query) -> Result<()> {
        return Ok({});
    }
}
