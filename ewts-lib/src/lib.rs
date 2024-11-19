mod converter;
mod dict;
mod tokenizer;

use converter::EwtsToUnicodeConverter;
use tokenizer::{EwtsToUnicodeTokenMap, EwtsToUnicodeTokenizer};

pub struct EwtsConverter {
    ewts_to_unicode_tokens_map: EwtsToUnicodeTokenMap,
}

impl EwtsConverter {
    pub fn create() -> Self {
        EwtsConverter {
            ewts_to_unicode_tokens_map: EwtsToUnicodeTokenMap::create(),
        }
    }

    pub fn ewts_to_unicode(&self, src: &str) -> String {
        let tokens = EwtsToUnicodeTokenizer::tokenize(&self.ewts_to_unicode_tokens_map, src);
        EwtsToUnicodeConverter::convert(&tokens)
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
        //(
        //    "@#/_/sangs rgyas chos dang tshogs kyi mchog rnams la/_/byang chub bar du bdag ni skyabs su mchi/_/bdag gyis spyin sogs bgyis pa'i bsod nams kyis/_/'gro la phan phyir sangs rgyas 'grub par shog_!",
        //    "༄༅། །སངས་རྒྱས་ཆོས་དང་ཚོགས་ཀྱི་མཆོག་རྣམས་ལ། །བྱང་ཆུབ་བར་དུ་བདག་ནི་སྐྱབས་སུ་མཆི། །བདག་གྱིས་སྤྱིན་སོགས་བགྱིས་པའི་བསོད་ནམས་ཀྱིས། །འགྲོ་ལ་ཕན་ཕྱིར་སངས་རྒྱས་འགྲུབ་པར་ཤོག ༈"
        //),
    ];

    #[test]
    fn ewts_to_unicode_converting_test() {
        let converter = EwtsConverter::create();

        TST_DATA.iter().for_each(|td| {
            assert_eq!(converter.ewts_to_unicode(td.0), td.1);
        });
    }
}
