# ewts

WASM-module to convert text from EWTS (Extended Wylie Transliteration Scheme) to Tibetan Unicode symbols

Fully compliant with the standard. See all rules on
[The Tibetan and Himalayan Library's site](https://www.thlib.org/reference/transliteration/#!essay=/thl/ewts/rules/) 
and tests on them here in [rules_test.rs](https://github.com/emgyrz/ewts-rs/blob/master/ewts/src/rules_test.rs) file.

It is part of set of apps/libs called **ewts-rs**.
See more [here](https://github.com/emgyrz/ewts-rs)


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



