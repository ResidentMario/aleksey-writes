#!/bin/bash
set -ex
RAW_PATH="/Users/alekseybilogur/Desktop/aleksey-writes-documents/raw/"
pushd /Users/alekseybilogur/Desktop/aleksey-writes/rust/html2documents/
for DOC in $(ls $RAW_PATH/website/); do
    cargo run \
        website \
        "$RAW_PATH/website/$DOC" \
        $(echo $DOC | sed -E "s/.html//") \
        --overwrite
done

for DOC in $(ls $RAW_PATH/spell/); do
    cargo run \
        spell \
        "$RAW_PATH/spell/$DOC" \
        $(echo $DOC | sed -E "s/.html//") \
        --overwrite
done

for DOC in $(ls $RAW_PATH/medium/); do
    cargo run \
        medium \
        "$RAW_PATH/medium/$DOC" \
        $(echo $DOC | sed -E "s/.html//") \
        --overwrite
done

for DOC in $(ls $RAW_PATH/kaggle/); do
    cargo run \
        kaggle \
        "$RAW_PATH/kaggle/$DOC" \
        $(echo $DOC | sed -E "s/.html//") \
        --overwrite
done
popd