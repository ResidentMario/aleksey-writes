use select::{document::Document as SelectDocument};
use select::predicate::{Name};
use std::io;
use std::{io::{Read, Write}, path::Path};
use std::{fs::File};
use std::{iter::Iterator};

pub type Result<T> = std::result::Result<T, HTML2DocumentsError>;

pub enum DocumentType {
    Medium,
    Website,
    Kaggle,
    Notion,
    Spell,
}

#[derive(Debug)]
pub enum HTML2DocumentsError {
    IOError(io::Error),
} 

pub struct Document {
    pub document_type: DocumentType,
    pub raw_html: SelectDocument,
    pub uid: String,
    pub plaintext: String,
}

impl Document {
    pub fn new(document_type: DocumentType, path: &str, uid: &str) -> Result<Document> {
        let parser = DocumentParser::new(document_type, path);
        let parser = match parser {
            Ok(p) => p,
            Err(e) => return Err(e),
        };
        let plaintext = parser.to_plaintext();
        let doc = Document { 
            document_type: parser.document_type,
            raw_html: parser.raw_html,
            uid: String::from(uid),
            plaintext: String::from(plaintext)
        };
        Ok(doc)
    }

    fn document_type_as_str(&self) -> &str {
        match self.document_type {
            DocumentType::Medium => "medium",
            DocumentType::Kaggle => "kaggle",
            DocumentType::Notion => "notion",
            DocumentType::Spell => "spell",
            DocumentType::Website => "website",
        }
    }

    pub fn write(&self, base_path: &str, overwrite: bool) -> Result<()> {
        let base_dir_path = Path::new(base_path);
        if !(base_dir_path.exists() && base_dir_path.is_dir()) {
            return Err(HTML2DocumentsError::IOError(
                io::Error::new(
                    io::ErrorKind::Other, 
                    "The given path does not exist or is not a directory."
                )
            ));
        }

        let document_type_path: String = base_path.to_owned() + "/" + self.document_type_as_str();
        if !Path::new(&document_type_path).exists() {
            create_dir(&document_type_path)?;
        }

        let documents_path: String = document_type_path + "/" + &self.uid;
        if Path::new(&documents_path).exists() {
            if overwrite {
                std::fs::remove_dir_all(&documents_path).map_err(|e| {
                    HTML2DocumentsError::IOError(e)
                })?;
                create_dir(&documents_path)?;
            }
            else {
                return Err(HTML2DocumentsError::IOError(
                    io::Error::new(
                        io::ErrorKind::Other, 
                        "The documents directory already exists and overwrite is set to false."
                    )
                ));
            }            
        }
        else {
            create_dir(&documents_path)?;
        }

        let plaintext_document_path: String = documents_path + "/" + "plaintext.txt";
        let mut fp = File::create(plaintext_document_path).map_err(|e| {
            HTML2DocumentsError::IOError(e)
        })?;
        fp.write_all(self.plaintext.as_bytes()).map_err(|e| {
            HTML2DocumentsError::IOError(e)
        })?;
        Ok(())
    }
}

pub trait Parser {
    fn new(document_type: DocumentType, path: &str) -> Result<DocumentParser> {
        let raw_html = read_file(path)?;
        Ok(DocumentParser { document_type, raw_html })
    }
    fn to_plaintext(&self) -> String;
}

pub struct DocumentParser {
    pub document_type: DocumentType,
    pub raw_html: SelectDocument,
}

impl Parser for DocumentParser {
    fn to_plaintext(&self) -> String {
        let result = match self.document_type {
            DocumentType::Medium => to_plaintext_medium(&self.raw_html),
            _ => String::from("Hello World!"),
        };
        result
    }
}

fn read_file(path: &str) -> Result<SelectDocument> {
    // Ensure the file exists and can be read by the executing user.
    let fp = Path::new(path);
    if !(fp.exists()) {
        return Err(HTML2DocumentsError::IOError(
            io::Error::new(io::ErrorKind::Other, "No document exists at the given path.")
        ));
    }

    let mut file = File::open(path).map_err(|e| { HTML2DocumentsError::IOError(e) })?;
    let mut contents = String::new();
    let read_result = file.read_to_string(&mut contents);
    read_result.map_err(|e| { HTML2DocumentsError::IOError(e) })?;

    // Actually read and return the document.
    let document = SelectDocument::from(contents.as_str());
    Ok(document)
}

fn create_dir(path: &str) -> Result<()> {
    std::fs::create_dir(path).map_err(|e| { HTML2DocumentsError::IOError(e) })
}

fn to_plaintext_medium(document: &SelectDocument) -> String {
    let mut paragraphs = vec![];
    document.find(Name("p")).for_each(|node| {
        paragraphs.push(node.text());
    });
    let result = paragraphs.into_iter().map(
        |t| { (t + " ").replace("“", "\"").replace("”", "\"") }
    ).collect();
    result
}