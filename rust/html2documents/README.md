# html2documents

Site-specific Rust package for converting an HTML document dumped to disk into a `plaintext` document dumped to a `txt` document in a `type/uid/` folder on disk.

Supports documents from the following list of valid `type` values:

* https://medium.com articles (`medium/`)
* https://residentmar.io blog posts (`website/`)
* https://spell.ml/blog/ blog posts (`spell/`)
* https://kaggle.com notebooks (`kaggle/`)

Will hopefully support the following one day soon:

* https://notion.so pages (`notion/`)

These are the types interesting to me personally because they are the services that host my various writings.

The document tree is saved to the `ResidentMario/aleksey-writes-documents` GH repo.

Usage:

```bash
$ cargo run medium /Users/alekseybilogur/Desktop/sf.html rubbish-2019 --overwrite
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/html2documents medium /Users/alekseybilogur/Desktop/aleksey-writes-documents/raw/medium/rubbish-san-fracisco.html rubbish-san-fracisco`
```

To run everything through use the script:

```bash
# PWD=[...]/scripts/
$ ./parse_all.sh
```
