#!/bin/bash

set -e

rename_main_files () {
  dir="${1}"

  mv "${dir}/ewts.js" "${dir}/index.js"
  mv "${dir}/ewts.d.ts" "${dir}/index.d.ts"
}

build_wasm_and_publish () {
  rm -rf ./pkg || true
  cargo build

  wasm-pack build --target bundler --out-dir pkg --release
  rename_main_files ./pkg

  wasm-pack build --target nodejs --out-dir pkg/nodejs --release
  rename_main_files ./pkg/nodejs
  rm ./pkg/nodejs/README.md ./pkg/nodejs/.gitignore

  wasm-pack build --target web --out-dir pkg/web --release
  rename_main_files ./pkg/web
  rm ./pkg/web/README.md ./pkg/web/.gitignore

  ./update_package_json.js

  wasm-pack pack
  wasm-pack publish
}


build_wasm_and_publish

