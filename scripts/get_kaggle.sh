#!/bin/bash
# Fetches all of my Kaggle kernels to disk and writes them to HTML for processing.
# Uses the Kaggle API.
set -ex

echo "Dumping the kernel list to disk..."
if [[ -f "kernels.txt" ]]; then
    rm kernels.txt
fi
touch kernels.txt
PYTHON_PARSE_SCRIPT=$(cat << EOF
import sys
import pandas as pd
import io
(pd.read_csv(io.StringIO(sys.stdin.read()), header=0)
 .iloc[:, 0]
 .pipe(lambda srs: srs[~(srs.str.contains('kernel') | srs.str.contains('notebook'))])
 .to_csv("kernels.txt", header=False, index=False, mode='a')
)
EOF
)
kaggle kernels list \
    --page-size 100 --page 1 \
    --user residentmario --kernel-type notebook --csv | \
    python -c "$PYTHON_PARSE_SCRIPT"
kaggle kernels list \
    --page-size 100 --page 2 \
    --user residentmario --kernel-type notebook --csv | \
    python -c "$PYTHON_PARSE_SCRIPT"
kaggle kernels list \
    --page-size 100 --page 3 \
    --user residentmario --kernel-type notebook --csv | \
    python -c "$PYTHON_PARSE_SCRIPT"
kaggle kernels list \
    --page-size 100 --page 4 \
    --user residentmario --kernel-type notebook --csv | \
    python -c "$PYTHON_PARSE_SCRIPT"

echo "Downloading kernels..."
rm -rf /tmp/kernels/
mkdir /tmp/kernels/
mkdir /tmp/kernels/raw/
xargs -n1 kaggle kernels pull --path /tmp/kernels/raw/ < kernels.txt

echo "Converting kernels from IPYNB to HTML..."
KERNELS_TXT_HOME=$PWD
KERNELS_RAW_HTML_HOME=$HOME/Desktop/aleksey-writes-documents/raw/kaggle/
rm -rf $KERNELS_RAW_HTML_HOME
mkdir $KERNELS_RAW_HTML_HOME
pushd /tmp/kernels/raw/

find . -regex './.*.ipynb' -exec jupyter nbconvert --to html --output-dir $KERNELS_RAW_HTML_HOME {} \;
popd

echo "Generating mapping..."
KERNELS_MAPPING_FILENAME=$HOME/Desktop/aleksey-writes-documents/mappings/kaggle.json
PYTHON_MAPPING_SCRIPT=$(cat << EOF
import pandas as pd
import json
kernels = pd.read_csv('kernels.txt')
kernels = list(
    map(
        lambda k: dict(
            uid=k.replace('residentmario/', '') + '.html',
            link='https://www.kaggle.com/' + k,
            backup='https://web.archive.org/web/https://www.kaggle.com/' + k
        ),
        kernels.values[:, 0].tolist()
    )
)
with open('$KERNELS_MAPPING_FILENAME', 'w') as fp:
    json.dump(kernels, fp, indent=4)
EOF
)
python -c "$PYTHON_MAPPING_SCRIPT"

echo "Cleaning up..."
rm -rf /tmp/kernels/
rm $KERNELS_TXT_HOME/kernels.txt

echo "Done!"