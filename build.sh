#!/bin/bash
#
# build.sh
# Builds wasm bundle for project_koskelantie web app.
#
wasm-pack build --target web --out-name wasm --out-dir ./static

