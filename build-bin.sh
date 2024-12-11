#!/bin/bash

set -e

get_version_of() { 
  echo $(cargo pkgid -p $1 | cut -d "#" -f2) 
}
bt() { 
  cross build -p ewts-cli -p ewts-c --release --target $1 
}
pack() {
  trg=$1
  fname=$2
  pack_prefix=$3
  version=$4
  tar -cz -C target "${trg}/release/${fname}" > "./packed_bins/${pack_prefix}_${version}_${trg}.tar.gz"
}

cli_version=$(get_version_of ewts-cli)
dylib_version=$(get_version_of ewts-c)

trg_l=x86_64-unknown-linux-gnu
# trg_l2=x86_64-unknown-linux-musl
trg_m=x86_64-apple-darwin
# trg_m2=aarch64-apple-darwin
trg_w=x86_64-pc-windows-gnu
# trg_w2=x86_64-pc-windows-msvc

rm -rf ./packed_bins
mkdir ./packed_bins

bt $trg_l
pack $trg_l ewts ewts-cli $cli_version
pack $trg_l libewts.so libewts.so $dylib_version

bt $trg_m
pack $trg_m ewts ewts-cli $cli_version
pack $trg_m libewts.dylib libewts.dylib $dylib_version

bt $trg_w
pack $trg_w ewts.exe ewts-cli $cli_version
pack $trg_w ewts.dll ewts.dll $dylib_version

