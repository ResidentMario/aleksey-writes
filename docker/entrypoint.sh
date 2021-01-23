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
echo "ES_PROCESS_ID IS $ES_PROCESS_PID"
terminate() {
    kill -s TERM "$ES_PROCESS_PID" 2>/dev/null
}
trap terminate SIGTERM
trap terminate SIGINT

# Wait for it to start.
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

# Push in data.
# TODO: write this bit. Probably via python -c.
# find /usr/local/documents/working/kaggle/ \
#     -regex ".*.txt" \
#     -exec jupyter nbconvert --to html --output-dir $KERNELS_RAW_HTML_HOME {} \;

# Wait for the child process to exit. This pattern of using wait is considered correct behavior
# in scripts; job control with fg, though useable by setting set -m, is recommended for
# interactive usage only.
wait "$ES_PROCESS_PID"