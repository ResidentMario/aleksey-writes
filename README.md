# aleksey-writes

A personal archival and retrieval service for my writing. Currently a work in progress.

## deployment (local)

1. Run the `get_kaggle.sh` and `parse_all.sh` scripts (in the `scripts/` directory).
2. Push the `aleksey-writes-documents` repository (that you just rebuilt above) to GitHub.
3. Build and serve the `elasticsearch` Docker container:

    ```bash
    # PWD=/path/to/aleksey-writes/docker
    # uses --no-cache b/c the documents are cached inside the image
    $ docker build . \
        --tag aleksey-writes/elasticsearch \
        --file Dockerfile.elasticsearch \
        --no-cache
    $ docker run --rm \
        -p 9200:9200 -p 9300:9300 \
        -e "discovery.type=single-node" \
        aleksey-writes/elasticsearch
    ```

4. Everything past that is a work in progress!
