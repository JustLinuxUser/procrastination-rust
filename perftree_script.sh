#!/usr/bin/bash

cd /home/andriy/Programming/langs/rust/procrastination-rust
echo perftree | cargo run --release --quiet "$@"
# 2> /dev/null
