use std::path::Path;
use std::io::{Read, Write};
use std::fs::File;
use std::iter::Iterator;

use select::{document::Document as SelectDocument, predicate::Predicate};
use snailquote::unescape;

pub mod err;
use err::{Result, HTML2DocumentsError};

pub enum DocumentType {
    Medium,
    Website,
    Kaggle,
    Notion,
    Spell,
}

pub struct Document {
    pub document_type: DocumentType,
    pub raw_html: SelectDocument,
    pub uid: String,
    pub plaintext: String,
}

impl Document {
    pub fn new(document_type: DocumentType, path: &str, uid: &str) -> Result<Document> {
        let parser = DocumentParser::new(document_type, path)?;
        let plaintext = parser.to_plaintext()?;
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
            return Err(
                HTML2DocumentsError::new_io_error(
                    "The given path does not exist or is not a directory."
                )
            );
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
                return Err(
                    HTML2DocumentsError::new_io_error(
                        "The documents dir exists and overwrite is false."
                    )
                );
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
    fn to_plaintext(&self) -> Result<String>;
}

pub struct DocumentParser {
    pub document_type: DocumentType,
    pub raw_html: SelectDocument,
}

impl Parser for DocumentParser {
    fn to_plaintext(&self) -> Result<String> {
        let result = match self.document_type {
            DocumentType::Medium => to_plaintext_medium(&self.raw_html),
            DocumentType::Kaggle => to_plaintext_kaggle(&self.raw_html),
            _ => Ok(String::from("Hello World!")),
        };
        result
    }
}

fn read_file(path: &str) -> Result<SelectDocument> {
    // Ensure the file exists and can be read by the executing user.
    let fp = Path::new(path);
    if !(fp.exists()) {
        return Err(
            HTML2DocumentsError::new_io_error(
                "The given path does not exist or is not a directory."
            )
        );
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

fn to_plaintext_medium(document: &SelectDocument) -> Result<String> {
    /*  Parsing Medium to plaintext is easy: just select the text inside the p tags on the page.
        There is a little bit junk text that comes along as well, but nothing too major.
     */
    let mut paragraphs = vec![];
    document.find(select::predicate::Name("p")).for_each(|node| {
        paragraphs.push(node.text());
    });
    let result = paragraphs.into_iter().map(
        // TODO: use Pattern syntax to make a single replace scan instead of two.
        |t| { (t + " ").replace("“", "\"").replace("”", "\"") }
    ).collect();
    Ok(result)
}

fn to_plaintext_kaggle(document: &SelectDocument) -> Result<String> {
    /*  Kaggle dumps the Jupyter source into the interior of a JS <script>, which executes
        Kaggle.State.push($JUPYTER_SOURCE), where $JUPYTER_SOURCE is a JSON Jupyter struct,
        escaped, as a string.

        I'm using "Kaggle.State.push(" as the opening sentinel and
        ");performance && performance.mark" as the closing sentinel. This may change if/when the
        Kaggle source code changes, it's not very robust, but it works well enough for my
        purposes, as I don't publish anything on Kaggle anymore anyway, so if it starts failing in
        the future I'm not much affected :).
     */
    let make_err = || {
        HTML2DocumentsError::new_parse_error(
            "Failed to parse Kaggle page input: the page provided did not meet the parser's 
            structural expectations. Perhaps the Kaggle page layout has changed?"
        )
    };
    let jupyter_script_node = document.find(
        select::predicate::Attr("id", "site-body")
        .descendant(select::predicate::Class("kaggle-component")))
        .next()
        .ok_or_else(make_err)?;
    let jupyter_script_node_text = jupyter_script_node.text();
    let (start_token, end_token) = ("Kaggle.State.push(", ");performance && performance.mark");
    let start_idx = jupyter_script_node_text
        .find(start_token)
        .ok_or_else(make_err)
        .map(|v| { v + start_token.len() })?;
    let end_idx = jupyter_script_node_text
        .find(end_token)
        .ok_or_else(make_err)?;
    
    let jupyter_script_node_text = &jupyter_script_node_text[start_idx..end_idx];
    let jupyter_script_node_text = 
        unescape(jupyter_script_node_text)
            .map_err(|e| { HTML2DocumentsError::new_parse_error(&format!("{:?}", e)) })?;
    println!("{:?}", jupyter_script_node_text);
    Ok(String::from("Hello World"))
}