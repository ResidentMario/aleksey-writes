use html2documents::{DocumentType, Document};

use structopt::StructOpt;

const DEFAULT_OUTPATH: &str = "/Users/alekseybilogur/Desktop/aleksey-writes-documents/working";
#[derive(StructOpt)]
/// Example usage:
///
/// ```bash
/// $ cargo run medium /Users/alekseybilogur/Desktop/sf.html rubbish-2019 --overwrite
/// ```
struct Cli {
    // https://stackoverflow.com/questions/54687403/how-can-i-use-enums-in-structopt
    document_type: DocumentType,
    path: std::path::PathBuf,
    uid: String,
    #[structopt(short = "o", long = "out", default_value = DEFAULT_OUTPATH)]
    out_path: String,
    #[structopt(long)]
    overwrite: bool,
}

fn main() {
    let args = Cli::from_args();
    let document = Document::new(
        args.document_type,
        // NOTE(aleksey): technically the following will fail if the path provided is a valid
        // non-UTF8 filepath. But that doesn't concern me so much, so I can be flippant here.
        &args.path.to_string_lossy().into_owned(),
        &args.uid,
    ).unwrap();
    document.write(
        &args.out_path,
        args.overwrite,
    ).unwrap();
}
