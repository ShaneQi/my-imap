DATA_PATH='/home/shane/server/my-imap/data'
SRC_PATH='/home/shane/server/my-imap'
docker run \
-it \
-v ${DATA_PATH}:/electricity-meter \
-v ${SRC_PATH}:/my-imap \
-v ${SRC_PATH}/.env:/.env \
-w /my-imap \
--name my-imap \
rust:1.51.0 \
/bin/bash -c \
"cargo run"

