DATA_PATH='/home/shane/server/persistent/electricity-meter'
SRC_PATH='/home/shane/server/my-imap'
docker run \
-d \
--name my-imap \
-v ${DATA_PATH}:/electricity-meter \
-v ${SRC_PATH}/.env:/.env \
ghcr.io/shaneqi/my-imap:latest

