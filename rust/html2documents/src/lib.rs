use select::{document::Document as SelectDocument};
use select::predicate::{Name};
use std::{io::{Read, Write}, path::Path};
use std::{fs::File};
use std::{iter::Iterator};

pub enum DocumentType {
    Medium,
    Website,
    Kaggle,
    Notion,
    Spell,
}

#[derive(Debug)]
pub enum HTML2DocumentsError {
    IOError(String),
} 

pub struct Document {
    pub document_type: DocumentType,
    pub raw_html: SelectDocument,
    pub uid: String,
    pub plaintext: String,
}

impl Document {
    pub fn new(document_type: DocumentType, path: &str, uid: &str)
        -> Result<Document, HTML2DocumentsError> {
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

    pub fn write(&self, base_path: &str, overwrite: bool) -> Result<(), HTML2DocumentsError> {
        // A metric butt-ton of directory creation and/or deletion boilerplate.
        // TODO: consolidate this stuff using helper functions.
        let base_dir_path = Path::new(base_path);
        if !(base_dir_path.exists() && base_dir_path.is_dir()) {
            return Err(HTML2DocumentsError::IOError(
                String::from("The given path does not exist or is not a directory.")
            ));
        }

        let document_type_path: String = base_path.to_owned() + "/" + self.document_type_as_str();
        if !Path::new(&document_type_path).exists() {
            create_dir(&document_type_path)?;
        }

        let documents_path: String = document_type_path + "/" + &self.uid;
        if Path::new(&documents_path).exists() {
            if overwrite {
                std::fs::remove_dir_all(&documents_path).map_err(|_| {
                    HTML2DocumentsError::IOError(
                        String::from("Error while trying to delete non-empty documents directory.")
                    )
                })?;
                create_dir(&documents_path)?;
            }
            else {
                return Err(HTML2DocumentsError::IOError(
                    String::from(
                        "The documents directory already exists and overwrite is set to false."
                    )
                ));
            }            
        }
        else {
            create_dir(&documents_path)?;
        }

        let plaintext_document_path: String = documents_path + "/" + "plaintext.txt";
        let mut fp = File::create(plaintext_document_path).map_err(|_| {
            HTML2DocumentsError::IOError(
                String::from("Error while trying to open plaintext document file.")
            )
        })?;
        fp.write_all(self.plaintext.as_bytes()).map_err(|_| {
            HTML2DocumentsError::IOError(
                String::from("Error while writing to plaintext document file.")
            )
        })?;
        Ok(())
    }
}

pub trait Parser {
    fn new(document_type: DocumentType, path: &str)
        -> Result<DocumentParser, HTML2DocumentsError> {
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

fn read_file(path: &str) -> Result<SelectDocument, HTML2DocumentsError> {
    // Ensure the file exists and can be read by the executing user.
    let fp = Path::new(path);
    if !(fp.exists()) {
        return Err(HTML2DocumentsError::IOError(
            String::from("No document exists at the given path.")
        ));
    }

    let mut file = File::open(path).map_err(
        |_| { HTML2DocumentsError::IOError(String::from("The given file could not be opened.")) }
    )?;
    let mut contents = String::new();
    let read_result = file.read_to_string(&mut contents);
    read_result.map_err(
        |_| { HTML2DocumentsError::IOError(String::from("The given file could not be read.")) }
    )?;

    // Actually read and return the document.
    let document = SelectDocument::from(contents.as_str());
    Ok(document)
}

fn create_dir(path: &str) -> Result<(), HTML2DocumentsError> {
    std::fs::create_dir(path).map_err(|_| {
        HTML2DocumentsError::IOError(
            String::from("Error while trying to create documents directory.")
        )
    })
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