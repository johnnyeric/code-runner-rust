# Build code-runner-rust

docker build -t code-runner-rust .
docker exec -it --rm code-runner-rust:latest sh

// https://www.shellhacks.com/docker-cp-command-copy-file-to-from-container/

docker cp <container-id>:/home/myapp/bin/code-runner-rust runner-alpine
docker cp f5c291c14c6b:/home/myapp/bin/code-runner-rust runner-alpine

echo '{ "language":"javascript",  "stdin": "1|2", "command":"ls", "files": [{"name": "main.js",            "content": "const fn"},{"name": "fn.js","content": "const fn"}]}' | ./code-runner-rust

echo '{ "language":"javascript",  "stdin": "1|2", "command":"", "files": [{"name": "main.js",            "content": "const fn"},{"name": "fn.js","content": "const fn"}]}' | ./code-runner-rust
