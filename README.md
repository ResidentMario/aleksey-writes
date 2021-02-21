# aleksey-writes

A personal archival and retrieval service for my writing, circa 2016-2020. A small side project I worked on for fun, and to help motivate me to learn a bit of Rust.

![](https://i.imgur.com/OqfmV9I.png)

![](https://i.imgur.com/lbaH5vH.png)

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

4. Build and serve the webapp:

    ```bash
    # PWD=/path/to/aleksey-writes/js
    $ npm run-script build:local
    $ nmp run-script serve:local
    ```
