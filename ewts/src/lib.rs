//!
//! Converter from EWTS (Extended Wylie Transliteration Scheme) to Tibetan Unicode symbols
//!
//! # Examples
//! ```
//! use ewts::{EwtsConverter};
//!
//! let converter = EwtsConverter::create();
//! let ewts_str = "oM aHhU~M` badz+ra gu ru pad+ma sid+d+hi hU~M`:";
//!
//! let tib_unicode_str = converter.ewts_to_unicode(ewts_str);
//!
//! assert_eq!(tib_unicode_str, "ཨོཾ་ཨཿཧཱུྂ་བཛྲ་གུ་རུ་པདྨ་སིདྡྷི་ཧཱུྂ༔");
//! ```
//!

mod converter;
mod dict;
mod tokenizer;

use converter::{EwtsToUnicodeConverter, EwtsToUnicodeConverterMaps};
use tokenizer::{EwtsToUnicodeTokenMap, EwtsToUnicodeTokenizer};

/// Main and only one exported value. Stores inner symbol's maps and functions
pub struct EwtsConverter {
    ewts_to_unicode_tokens_map: EwtsToUnicodeTokenMap,
    ewts_to_unicode_converter_maps: EwtsToUnicodeConverterMaps,
}

impl EwtsConverter {
    /// Creates one
    pub fn create() -> Self {
        EwtsConverter {
            ewts_to_unicode_tokens_map: EwtsToUnicodeTokenMap::create(),
            ewts_to_unicode_converter_maps: EwtsToUnicodeConverterMaps::create(),
        }
    }

    /// Converts ewts symbols to Tibetan unicode.
    /// e.g. "ka " -> "ཀ་"; "sgrol " -> "སྒྲོལ་"
    pub fn ewts_to_unicode(&self, src: &str) -> String {
        let tokens = EwtsToUnicodeTokenizer::tokenize(&self.ewts_to_unicode_tokens_map, src);
        EwtsToUnicodeConverter::convert(&self.ewts_to_unicode_converter_maps, &tokens, src)
    }
}

/////
/////
/////
/////
/////
#[cfg(test)]
mod tests {
    use super::*;

    static TST_DATA: &[(&str, &str)] = &[
        ("rkw rk+w ", "རྐཝ་རྐྭ་"),
        (
            r#"
rka rga rnga rja rnya rta rda rna rba rma rtsa rdza 
lka lga lnga lca lja lta lda lpa lba lha 
ska sga snga snya sta sda sna spa sba sma stsa 
kwa khwa gwa cwa nywa twa dwa tswa tshwa zhwa zwa 
rwa shwa swa hwa 
kya khya gya pya phya bya mya 
kra khra gra tra thra dra pra phra bra mra shra sra hra 
kla gla bla zla rla sla 
rkya rgya rmya rgwa rtswa 
skya sgya spya sbya smya 
skra sgra snra spra sbra smra 
grwa drwa phywa 
"#,
            r#"
རྐ་རྒ་རྔ་རྗ་རྙ་རྟ་རྡ་རྣ་རྦ་རྨ་རྩ་རྫ་
ལྐ་ལྒ་ལྔ་ལྕ་ལྗ་ལྟ་ལྡ་ལྤ་ལྦ་ལྷ་
སྐ་སྒ་སྔ་སྙ་སྟ་སྡ་སྣ་སྤ་སྦ་སྨ་སྩ་
ཀྭ་ཁྭ་གྭ་ཅྭ་ཉྭ་ཏྭ་དྭ་ཙྭ་ཚྭ་ཞྭ་ཟྭ་
རྭ་ཤྭ་སྭ་ཧྭ་
ཀྱ་ཁྱ་གྱ་པྱ་ཕྱ་བྱ་མྱ་
ཀྲ་ཁྲ་གྲ་ཏྲ་ཐྲ་དྲ་པྲ་ཕྲ་བྲ་མྲ་ཤྲ་སྲ་ཧྲ་
ཀླ་གླ་བླ་ཟླ་རླ་སླ་
རྐྱ་རྒྱ་རྨྱ་རྒྭ་རྩྭ་
སྐྱ་སྒྱ་སྤྱ་སྦྱ་སྨྱ་
སྐྲ་སྒྲ་སྣྲ་སྤྲ་སྦྲ་སྨྲ་
གྲྭ་དྲྭ་ཕྱྭ་
"#,
        ),
        ("rgyi", "རྒྱི"),
        (
            "oM aHhU~M` badz+ra gu ru pad+ma sid+d+hi hU~M`:",
            "ཨོཾ་ཨཿཧཱུྂ་བཛྲ་གུ་རུ་པདྨ་སིདྡྷི་ཧཱུྂ༔",
        ),
        ("g.yeng gyeng ", "གཡེང་གྱེང་"),
        //("va fi &ung","བ༹་ཕི༹་྅ུང" ),
        ("sha ai gaang angs ", "ཤ་ཨཻ་གཨང་ཨངས་"),
        ("sat+t+wa", "སཏྟྭ"),    // rule 4
        ("bre+u rdo+e ", "བྲེུ་རྡོེ་"),    // rule 5
        ("rta r+ta ", "རྟ་རྟ་"), // rule 6
        ("R+na R+Ya R+ya ", "ཪྣ་ཪྻ་ཪྱ་"), // rule 13
        ("sarba mang+galaM ", "སརྦ་མངྒལཾ་"), // rule 7
        (
            "@#/_/sangs rgyas chos dang tshogs kyi mchog rnams la/_/byang chub bar du bdag ni skyabs su mchi/_/bdag gyis spyin sogs bgyis pa'i bsod nams kyis/_/'gro la phan phyir sangs rgyas 'grub par shog_!",
            "༄༅། །སངས་རྒྱས་ཆོས་དང་ཚོགས་ཀྱི་མཆོག་རྣམས་ལ། །བྱང་ཆུབ་བར་དུ་བདག་ནི་སྐྱབས་སུ་མཆི། །བདག་གྱིས་སྤྱིན་སོགས་བགྱིས་པའི་བསོད་ནམས་ཀྱིས། །འགྲོ་ལ་ཕན་ཕྱིར་སངས་རྒྱས་འགྲུབ་པར་ཤོག ༈"
        ),
    ];

    #[test]
    fn ewts_to_unicode_converting_test() {
        let converter = EwtsConverter::create();

        TST_DATA.iter().for_each(|td| {
            assert_eq!(converter.ewts_to_unicode(td.0), td.1);
        });
    }

    #[test]
    fn ewts_to_unicode_non_tibetan_test() {
        let data = [
            ("k [alphabet]_h ", "ཀ་alphabet ཧ་"),
            ("[u]", "u"),
            ("_[u]rjes[U]_[\\]_", " uརྗེསU \\ "),
        ];

        data.iter().for_each(|td| {
            let converter = EwtsConverter::create();
            assert_eq!(converter.ewts_to_unicode(td.0), td.1);
        });
    }
}
