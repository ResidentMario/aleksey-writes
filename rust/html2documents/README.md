# html2documents

Site-specific Rust package for converting HTML documents dumped to disk into `(plaintext, simplified_html)` pairs dumped to `(txt, html)` documents in a `type/uid/` folder on disk.

Supports documents from the following list of valid `type` values:

* https://medium.com articles (`medium/`)
* https://residentmar.io blog posts (`website/`)
* https://kaggle.com notebooks (`kaggle/`)
* https://notion.so pages (`notion/`)
* https://spell.ml/blog/ blog posts (`spell/`)

These are the types interesting to me personally because they are the services that host my various writings.

The document tree is saved to the `ResidentMario/aleksey-writes-documents` GH repo. The `txt` documents are used by the Elasticsearch indexer, used for search. The `html` documents are used by the webapp.

Usage:

```bash
$ html2documents post.html medium \
    $HOME/Desktop/aleksey-writes-documents
```
