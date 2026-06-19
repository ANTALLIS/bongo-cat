#!/bin/bash
set -e

cargo build --release --bins
cargo bundle --release

APP="target/release/bundle/osx/BongoCat.app"
cp "target/release/bongo-listener" "$APP/Contents/MacOS/"

