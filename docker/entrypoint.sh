#!/bin/bash
# Initialize the ES process in the background.
bash bin/docker-entrypoint.sh &

# Docker will send SIGTERM to this process at docker stop time. Interactive use of Ctrl+C will
# send SIGINT. The default signal handler behavior in Bash is that it will accrue signals
# indefinitely while there are process forks still running, and not respond to them until the
# subprocess exits. To make this process respect signals properly, we need to define and use our
# own signal handler, which terminates the child process.
# Cf. https://unix.stackexchange.com/questions/146756/forward-sigterm-to-child-in-bash
ES_PROCESS_PID=$!
terminate() {
    kill -s TERM "$ES_PROCESS_PID" 2>/dev/null
}
trap terminate SIGTERM
trap terminate SIGINT

# Wait for ES to start.
sleep 5
while true
do
    curl -X GET "localhost:9200/_cat/nodes?v=true&pretty" 1>/dev/null 2>/dev/null
    if [ $? -eq 0 ]; then
        break
    else
        echo "ElasticSearch still not up, waiting another 5 seconds..."
        sleep 5
    fi
done

# Populate ES.
echo "Populating ElasticSearch..."
PYTHON_WRITE_SCRIPT=$(cat << EOF
import json
import requests
document_idx = 1
for document_type in ['website', 'spell', 'medium', 'kaggle']:
    with open('/usr/local/documents/mappings/' + document_type + '.json', 'r') as fp:
        mappings = json.load(fp)
        for mapping in mappings:
            uid = mapping['uid'].replace('.html', '')
            mapping['document_type'] = document_type
            plaintext = (
                '/usr/local/documents/working/' + document_type + '/' + uid + '/plaintext.txt'
            )
            with open(plaintext, 'r') as fp:
                mapping['content'] = fp.read()
            requests.post(
                'http://localhost:9200/document/_doc/' + str(document_idx) + '?pretty', json=mapping
            )
            document_idx += 1
print('Done populating ElasticSearch 😎. Wrote ' + str(document_idx) + ' documents total.')
EOF
)
python3 -c "$PYTHON_WRITE_SCRIPT"

# Wait for the child process to exit. This pattern of using wait is considered correct behavior
# in scripts; job control with fg, though useable by setting set -m, is recommended for
# interactive usage only.
wait "$ES_PROCESS_PID"