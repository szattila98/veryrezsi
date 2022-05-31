#!/bin/sh
cd ../../app
cargo install cargo-watch
cargo watch --why -x run &
child=$!

shutdown() {
    kill $child
}

trap 'shutdown' TERM

wait $child
