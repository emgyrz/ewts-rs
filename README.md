# ewts-rs

Converter from EWTS (Extended Wylie Transliteration Scheme) to Tibetan Unicode symbols

Fully compliant with the standard. See all rules on
[The Tibetan and Himalayan Library's site](https://www.thlib.org/reference/transliteration/#!essay=/thl/ewts/rules/) 
and tests on them here in [rules_test.rs](https://github.com/emgyrz/ewts-rs/blob/master/ewts/src/rules_test.rs) file.

**ewts-rs** includes several separate parts:

## ewts [![Crates.io Version](https://img.shields.io/crates/v/ewts)](https://crates.io/crates/ewts)
Core conversion library written in Rust.

[\[Docs\]](https://docs.rs/ewts/latest/ewts/)

#### Example:
```rust
use ewts::{EwtsConverter};

let converter = EwtsConverter::create();
let ewts_str = "oM aHhU~M` badz+ra gu ru pad+ma sid+d+hi hU~M`:";

let tib_unicode_str = converter.ewts_to_unicode(ewts_str);

assert_eq!(tib_unicode_str, "ཨོཾ་ཨཿཧཱུྂ་བཛྲ་གུ་རུ་པདྨ་སིདྡྷི་ཧཱུྂ༔");
```

## ewts-cli [![Crates.io Version](https://img.shields.io/crates/v/ewts-cli)](https://crates.io/crates/ewts-cli)
Command line interface for conversion. For use in your favorite console 

#### Example:
```sh
$ ewts --input "bkra shis bde legs/"
# བཀྲ་ཤིས་བདེ་ལེགས།

$ ewts --help
# ...
# Usage: ewts [OPTIONS] --input <INPUT>
# 
# Options:
#   -s, --source-type <SOURCE_TYPE>  Type of input symbols [default: ewts] [possible values: ewts, unicode]
#   -i, --input <INPUT>              String to convert
#   -h, --help                       Print help
#   -V, --version                    Print version

# to convert file:
$ ewts -i "$(cat /path/to/your/file.txt)"
```


#### Usage
https://github.com/user-attachments/assets/28f62452-abf4-437f-b973-666a6a36403e


#### Installation
For now only with
```sh
cargo install ewts-cli
```


## ewts-wasm
WASM-module for use in a browser or somewhere else

_TODO_


## References
- Ewts symbols [table](https://www.thlib.org/reference/transliteration/#!essay=/thl/ewts/tables/)
- The dictionary was taken from [here](https://github.com/rogerespel/ewts-js/blob/main/src/EwtsConverter.mjs)

## Misc
This converter does not perform any checks, substitutions, transformations - if you have written incorrectly, 
you will get incorrect characters in the result.

