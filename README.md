# ewts-rs

Converter from EWTS (Extended Wylie Transliteration Scheme) to Tibetan Unicode symbols. 
Written in Rust and can be used, of cource, as Rust library, also as CLI tool and in JS-environment.



<p align="center">
  <br />
  <kbd><img width="300" src="https://github.com/user-attachments/assets/a1d52bcf-70ea-42b9-87d0-0dee860988d9"></kbd>
  <h4 align="center"><a href="https://emgyrz.github.io/ewts-rs/">See demo page</a></h4>
  <br />
</p>

Fully compliant with the standard. See all rules on
[The Tibetan and Himalayan Library's site](https://www.thlib.org/reference/transliteration/#!essay=/thl/ewts/rules/) 
and tests on them here in [rules_test.rs](https://github.com/emgyrz/ewts-rs/blob/master/ewts/src/rules_test.rs) file.

> [!IMPORTANT]  
> Currently, only the conversion from EWTS to Tibetan Unicode is implemented. The conversion in the opposite direction will be coming soon.

<br />
**ewts-rs** includes several separate parts:

## ewts [![Crates.io Version](https://img.shields.io/crates/v/ewts)](https://crates.io/crates/ewts)
Core conversion library.

[\[rust docs\]](https://docs.rs/ewts/latest/ewts/)

#### Example:
```rust
use ewts::EwtsConverter;

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


#### Demo
https://github.com/user-attachments/assets/28f62452-abf4-437f-b973-666a6a36403e


#### Installation
For now only with
```sh
cargo install ewts-cli
```


## ewts-wasm [![Npm Version](https://img.shields.io/npm/v/ewts)](https://www.npmjs.com/package/ewts)
WASM-module for using in browser, nodejs or somewhere else.

See details in [ewts-wasm/README.md](https://github.com/emgyrz/ewts-rs/tree/master/ewts-wasm).

#### Installation
```sh
npm install ewts
```

#### Usage
```javascript
import {EwtsConverter} from 'ewts'

const converter = new EwtsConverter()

const ewtsStr = "oM ma Ni pad+me hU~M/"

const tibUnicodeStr = converter.ewtsToUnicode(ewtsStr)

console.log(tibUnicodeStr)
// "ཨོཾ་མ་ཎི་པདྨེ་ཧཱུྃ།"

```


## Speed comparison with other converters

I do not know who will need to transliterate large amounts of text, 
but still I want to mention the difference in the speed of some implementations. 
Especially considering that the current implementation is several times faster than others 
(from 12 to almost 100 times).


| Tool                                                                 | Speed               | Launch code                                                                                        |
| -------------------------------------------------------------------- | ------------------- | -------------------------------------------------------------------------------------------------- |
| ewts-rs (in rust)                                                    | ~26491 Kb/s (1.00x) | [bench/main.rs](https://github.com/emgyrz/ewts-rs/blob/master/bench/src/main.rs)                   |
| [jsewts](https://github.com/buda-base/jsewts)                        | ~2141 Kb/s (12.4x)  | [js_tools_bench.js](https://github.com/emgyrz/ewts-rs/blob/master/bench/js_tools_bench.js)         |
| [ewts-js](https://github.com/rogerespel/ewts-js)                     | ~1941 Kb/s (13.6x)  | [js_tools_bench.js](https://github.com/emgyrz/ewts-rs/blob/master/bench/js_tools_bench.js)         |
| ewts-rs (as wasm)                                                    | ~14598 Kb/s (1.81x) | [js_tools_bench.js](https://github.com/emgyrz/ewts-rs/blob/master/bench/js_tools_bench.js)         |
| [ewts-converter (java)](https://github.com/buda-base/ewts-converter) | ~1822 Kb/s (14.54x) | [java_bench.java](https://github.com/emgyrz/ewts-rs/blob/master/bench/java_bench.java)             |
| [pyewts](https://github.com/OpenPecha/pyewts)                        | ~274 Kb/s (96.7x)   | [python_tools_bench.py](https://github.com/emgyrz/ewts-rs/blob/master/bench/python_tools_bench.py) |

A little bit more info is at [bench/README](https://github.com/emgyrz/ewts-rs/tree/master/bench)



## References
- Ewts symbols [table](https://www.thlib.org/reference/transliteration/#!essay=/thl/ewts/tables/).
- Initially, the character matches for  are taken 
  from [here](https://github.com/rogerespel/ewts-js/blob/main/src/EwtsConverter.mjs).
  Thanks for not having to type them manually.

## Misc
- This converter does not perform any checks, substitutions, transformations - if you have written incorrectly, 
you will get incorrect characters in the result.


