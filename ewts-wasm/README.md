# ewts

WASM-module to convert text from EWTS (Extended Wylie Transliteration Scheme) to Tibetan Unicode symbols

Fully compliant with the standard. See all rules on
[The Tibetan and Himalayan Library's site](https://www.thlib.org/reference/transliteration/#!essay=/thl/ewts/rules/) 
and tests on them here in [rules_test.rs](https://github.com/emgyrz/ewts-rs/blob/master/ewts/src/rules_test.rs) file.

It is part of set of apps/libs called **ewts-rs**.
See more [here](https://github.com/emgyrz/ewts-rs)


## Installation
```sh
npm install ewts
```

## Usage
Main module in package (`'ewts/index.js`') at this time can only be used with bundlers (webpack, rollup, etc.).
If you don't use them, you can import submodule that natively supported by nodejs(`ewts/nodejs`) 
or by browsers (`'ewts/web'`)

More info about difference between submodules in `ewts` package can be found 
on [wasm-pack documentation](https://rustwasm.github.io/wasm-pack/book/commands/build.html#target):
- `ewts`(`ewts/index.js` etc.) built with `--target bundler` flag
- `ewts/nodejs` with `--target nodejs`
- and `ewts/web` built with `--target web` flag


```javascript
import {EwtsConverter} from 'ewts'
//                  or from 'ewts/nodejs'
//                  or from 'ewts/web'

const converter = new EwtsConverter()

const ewtsStr = "oM ma Ni pad+me hU~M/"

const tibUnicodeStr = converter.ewtsToUnicode(ewtsStr)

console.log(tibUnicodeStr)
// "ཨོཾ་མ་ཎི་པདྨེ་ཧཱུྃ།"
```

#### Examples
Library usage examples stored at [./examples](https://github.com/emgyrz/ewts-rs/blob/master/ewts-wasm/examples) dir.

To view it at work do:
```sh
git clone https://github.com/emgyrz/ewts-rs.git
cd ewts-rs/ewts-wasm/examples

npm i

# prints to console conversion result
node ./usage_in_node.js

# opens browser with simple page where `ewts` is used
npx http-server . -o "/usage_in_browser_without_bundlers.html"
```
