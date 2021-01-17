use html2documents::{DocumentType, DocumentParser, Document, Parser};

fn main() {
    // let dp = DocumentParser::new(
    //     DocumentType::Medium, "/Users/alekseybilogur/Desktop/sf.html"
    // ).unwrap();
    // println!("Result is: {}", dp.to_plaintext());
    let document = Document::new(
        DocumentType::Medium,
        "/Users/alekseybilogur/Desktop/sf.html",
        "rubbish-2019"
    ).unwrap();
    document.write(
        "/Users/alekseybilogur/Desktop/aleksey-writes-documents",
        true,
    ).unwrap();
}
