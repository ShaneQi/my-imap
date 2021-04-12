docker run \
-d \
-v /volume2/docker/splunk/data/electricity-meter:/electricity-meter \
-v /volume2/docker/my-imap:/my-imap \
-w /my-imap \
--name my-imap \
rust:1.51.0 \
/bin/bash -c \
"cargo run --release"

