DATA_PATH='/Users/shane/Server/splunk/data/electricity-meter'
SRC_PATH='/Users/shane/Server/my-imap'
docker run \
-d \
-v ${DATA_PATH}:/electricity-meter \
-v ${SRC_PATH}:/my-imap \
-w /my-imap \
--name my-imap \
rust:1.51.0 \
/bin/bash -c \
"cargo run --release"

