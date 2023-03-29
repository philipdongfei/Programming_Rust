#!/bin/bash
echo "Hello, world" > test.txt
cargo run "world" "Rust" test.txt test-modified.txt
cat test-modified.txt
#error handling
cargo run "[[a-z]" "0" test.txt test-modified.txt
