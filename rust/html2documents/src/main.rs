use html2documents::{DocumentType, Document};

fn main() {
    // let document = Document::new(
    //     DocumentType::Medium,
    //     "/Users/alekseybilogur/Desktop/sf.html",
    //     "rubbish-2019"
    // ).unwrap();
    // document.write(
    //     "/Users/alekseybilogur/Desktop/aleksey-writes-documents",
    //     true,
    // ).unwrap();
    let document = Document::new(
        DocumentType::Kaggle,
        "/Users/alekseybilogur/Desktop/s3.html",
        "s3-intelligent-tiering"
    ).unwrap();
    document.write(
        "/Users/alekseybilogur/Desktop/aleksey-writes-documents",
        true,
    ).unwrap();
}
