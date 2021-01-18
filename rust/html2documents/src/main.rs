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
    // let document = Document::new(
    //     DocumentType::Spell,
    //     "/Users/alekseybilogur/Desktop/mixed-precision.html",
    //     "mixed-precision-training"
    // ).unwrap();
    // document.write(
    //     "/Users/alekseybilogur/Desktop/aleksey-writes-documents",
    //     true,
    // ).unwrap();
    // let document = Document::new(
    //     DocumentType::Website,
    //     "/Users/alekseybilogur/Desktop/python-mixins.html",
    //     "python-mixins"
    // ).unwrap();
    // document.write(
    //     "/Users/alekseybilogur/Desktop/aleksey-writes-documents",
    //     true,
    // ).unwrap();
    let document = Document::new(
        DocumentType::Notion,
        "/Users/alekseybilogur/Desktop/ludwig.html",
        "ludwig-notes"
    ).unwrap();
    document.write(
        "/Users/alekseybilogur/Desktop/aleksey-writes-documents",
        true,
    ).unwrap();
}
