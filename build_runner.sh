#!/bin/bash

# cargo build --target x86_64-unknown-linux-musl --release

docker build -t code-runner-rust .

# need to create container and extract runner from it
# docker exec -it --rm code-runner-rust:latest sh
# docker cp <container-id>:/home/myapp/bin/code-runner-rust runner-alpine