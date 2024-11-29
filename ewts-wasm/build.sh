#!/bin/bash

set -e

build_wasm_and_publish () {
  rm -rf ./pkg || true
  cargo build
  wasm-pack build --target bundler --out-dir pkg

  mv pkg/ewts.js pkg/index.js
  mv pkg/ewts.d.ts pkg/index.d.ts

  wasm-pack build --target nodejs  --out-dir pkg/nodejs
  mv pkg/nodejs/ewts.js pkg/index.node.js
  rm -rf pkg/nodejs

  wasm-pack build --target web  --out-dir pkg/web
  mv pkg/web/ewts.js pkg/index.web.js
  rm -rf pkg/web
  

  ./update_package_json.js

  wasm-pack pack
  wasm-pack publish
}


build_wasm_and_publish

