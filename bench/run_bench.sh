#!/bin/bash

mkdir ./deps

echo "EWTS benchmarks: preparing..."
cargo build --release
npm i
pip install pyewts -t ./deps/

prepare_java () {
  if [ ! -f ./deps/commons-lang3.tar.gz ]; then
    curl --output ./deps/commons-lang3.tar.gz  "https://dlcdn.apache.org//commons/lang/binaries/commons-lang3-3.17.0-bin.tar.gz"
    tar -xzf ./deps/commons-lang3.tar.gz -C ./deps/
  fi
  git clone https://github.com/buda-base/ewts-converter.git --depth=1 ./deps/java_converter
  javac -d ./deps/java_classes -classpath ./deps/commons-lang3-3.17.0/commons-lang3-3.17.0.jar: ./deps/java_converter/src/main/java/io/bdrc/ewtsconverter/*
  cd ./deps/java_classes/
  jar cvf ../EwtsConverter.jar *
  cd -
}
prepare_java



echo -e "\nEWTS benchmarks:"

../target/release/bench
./js_tools_bench.js
java -classpath deps/commons-lang3-3.17.0/commons-lang3-3.17.0.jar:deps/EwtsConverter.jar: java_bench.java
./python_tools_bench.py



