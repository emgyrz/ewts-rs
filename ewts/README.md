# ewts

[![Crates.io Version](https://img.shields.io/crates/v/ewts)](https://crates.io/crates/ewts)

Library to convert text from EWTS (Extended Wylie Transliteration Scheme) to Tibetan Unicode symbols

Fully compliant with the standard. See all rules on
[The Tibetan and Himalayan Library's site](https://www.thlib.org/reference/transliteration/#!essay=/thl/ewts/rules/) 
and tests on them here in [rules_test.rs](https://github.com/emgyrz/ewts-rs/blob/master/ewts/src/rules_test.rs) file.

It is part of set of apps/libs called **ewts-rs**.
See more [here](https://github.com/emgyrz/ewts-rs)

[\[Docs\]](https://docs.rs/ewts/latest/ewts/)

#### Example:
```rust
use ewts::{EwtsConverter};

let converter = EwtsConverter::create();
let ewts_str = "oM aHhU~M` badz+ra gu ru pad+ma sid+d+hi hU~M`:";

let tib_unicode_str = converter.ewts_to_unicode(ewts_str);

assert_eq!(tib_unicode_str, "ཨོཾ་ཨཿཧཱུྂ་བཛྲ་གུ་རུ་པདྨ་སིདྡྷི་ཧཱུྂ༔");
```


## References
- Ewts symbols [table](https://www.thlib.org/reference/transliteration/#!essay=/thl/ewts/tables/)
- The dictionary was taken from [here](https://github.com/rogerespel/ewts-js/blob/main/src/EwtsConverter.mjs)

## Misc
This converter does not perform any checks, substitutions, transformations - if you have written incorrectly, 
you will get incorrect characters in the result.

