#!/bin/bash

set -e

get_version_of() { 
  echo $(cargo pkgid -p $1 | cut -d "#" -f2) 
}
pack() {
  trg=$1
  cli_name=$2
  lib_name=$3

  cross build -p ewts-cli -p ewts-c --release --target $trg

  cd ./target
  tar -cz "${trg}/release/${cli_name}" > "../packed_bins/ewts-cli_${cli_version}_${trg}.tar.gz"
  tar -cz *.h "${trg}/release/${lib_name}" > "../packed_bins/${lib_name}_${dylib_version}_${trg}.tar.gz"
  cd -
}

cli_version=$(get_version_of ewts-cli)
dylib_version=$(get_version_of ewts-c)

rm -rf ./packed_bins
mkdir ./packed_bins

cp ./ewts-c/*.h "./target/"

pack x86_64-unknown-linux-gnu ewts libewts.so
pack x86_64-apple-darwin ewts libewts.dylib
pack aarch64-apple-darwin ewts libewts.dylib
pack x86_64-pc-windows-gnu ewts.exe ewts.dll

