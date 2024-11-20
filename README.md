# ewts-rs

Converter from EWTS (Extended Wylie Transliteration Scheme) to Tibetan Unicode symbols

## _in development_

## Parts
**ewts-rs** includes several separate parts:

#### ewts [![Crates.io Version](https://img.shields.io/crates/v/ewts)](https://crates.io/crates/ewts)
Core conversion library written in Rust.

[\[Docs\]](https://docs.rs/ewts/latest/ewts/)

#### ewts-cli [![Crates.io Version](https://img.shields.io/crates/v/ewts-cli)](https://crates.io/crates/ewts-cli)
Command line interface for conversion. For use in your favorite console 


#### ewts-wasm
WASM-module for use in a browser or somewhere else

_TODO_


## References
- Ewts symbols [table](https://www.thlib.org/reference/transliteration/#!essay=/thl/ewts/tables/)
- The dictionary was taken from [here](https://github.com/rogerespel/ewts-js/blob/main/src/EwtsConverter.mjs)

## Misc
This converter does not perform any checks, substitutions, transformations - if you have written incorrectly, 
you will get incorrect characters in the result.

