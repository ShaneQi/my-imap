DATA_PATH='/home/shane/server/persistent/electricity-meter'
SRC_PATH='/home/shane/server/my-imap'
docker run \
-d \
-v ${DATA_PATH}:/electricity-meter \
-v ${SRC_PATH}:/my-imap \
-w /my-imap \
--name my-imap \
rust:1.51.0 \
/bin/bash -c \
"cargo run --release"

