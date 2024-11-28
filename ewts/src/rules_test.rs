///
/// EWTS Rules Tests
/// https://www.thlib.org/reference/transliteration/#!essay=/thl/ewts/rules/
///
static RULES_TST_DATA: &[(&str, &str)] = &[
    //
    // Rule 1:
    // Transliterate Tibetan characters in a syllable from left to right
    // and in stacks from top to bottom with the vowel being transliterated
    // after the final consonant of the root letter or stack.
    // Equivalents for characters are in the charts below.
    ("bsgribs", "བསྒྲིབས"),
    //
    // Rule 2:
    // If there is no explicit vowel mark, the implicit vowel is transliterated as “a”
    // and placed after the final consonant of the root letter or stack.
    ("mkhan", "མཁན"),
    //
    // Rule 3:
    // Use the period to horizontally display two consonants that would normally be stacked.
    ("gyon g.yon ", "གྱོན་གཡོན་"),
    //
    // Rule 4:
    // The use of the plus-sign (“+”) is required between consonants in a non-standard Tibetan stack.
    ("sat+t+wa", "སཏྟྭ"),
    //
    // Rule 5:
    // Use the plus-sign (“+”) between transliteration equivalents for multiple vowel signs above
    // and/or below the same Tibetan stack. In such cases, the vowels should be transliterated
    // from bottom to top even though this may contradict the logical order of the expanded phrase.
    ("bru+e rdo+e ", "བ\u{fb2}\u{f74}\u{f7a}་ར\u{fa1}\u{f7c}\u{f7a}་"),
    //
    // Rule 6:
    // The transliteration of a standard Tibetan stack that uses the plus-sign (“+”) is equivalent
    // to the transliteration that does not.
    ("rta r+ta ", "རྟ་རྟ་"),
    //
    // Rule 7:
    // For Tibetan transliterations of multi-syllable Sanskrit words that fall within
    // a single tsheg bar (Tibetan “syllable”), the implicit vowel, “a,” should be inserted
    // after each cluster consonant without an explicit vowel mark except when the virama (Tib., srog med)
    // is subscribed to that cluster. If the word ends in an anusvara (“M”) or a visarga (“H”) the final “a”
    // is inserted before their transliteration.
    ("sarba mang+galaM ", "སརྦ་མངྒལཾ་"),
    //
    // Rule 8:
    // All characters can be represented by the escape sequence “\u” plus their 4-digit hexadecimal code
    // for standard Unicode characters. For surrogate pairs, the escape sequence “\U” plus the 8-digit hexadecimal code
    // should be used. In either case, the full 4 or 8 hexadecimal code must be used without dropping leading zeros.
    // The characters in the list of those not found in Unicode 4.0 have been assigned values in the Private Use Area,
    // so that the standard escape sequence, “\uXXXX,” can be used.
    ("ka \\u0F40", "ཀ་ཀ"),
    //
    // Rule 9:
    // To insert a run of non-Tibetan characters within Tibetan transliteration: the whole string,
    // encoded in UTF-8, must be enclosed in brackets. Pairs of opening and closing brackets may be nested
    // with the final closing bracket indicating the resumption of Tibetan transliteration.
    // The escape sequences “\uXXXX” and “\UXXXXXXXX” can be used within brackets to refer to Tibetan
    // or non-Tibetan characters.
    ("khong [New York] la phebs song /", "ཁོང་New York་ལ་ཕེབས་སོང་།"),
    //
    // Rule 10:
    // To insert a single non-Tibetan character, numeral, or punctuation mark within a run of transliterated Tibetan,
    // prefix it with a backslash. (Note: The upper or lowercase “u” cannot be inserted through this method,
    // since “\u” and “\U” trigger the insertion of Unicode characters by their hexadecimal value.
    // Brackets must be used to insert a single letter “u” or “U,” e.g. [u] or [U].)
    ("de la \\3 yod/", "དེ་ལ་3་ཡོད།"),
    //
    // Rule 11:
    // When the a-chen (“big a”) is found at the beginning of a word and lacks a vowel sign,
    // it is transliterated as “a.” Otherwise, it is transliterated according to the vowel sign attached to it.
    // If it is found in the middle of a stack, transliterate it as “+a”;
    // if it is found in the middle of a syllable (tsheg bar), transliterate it as “.a”.
    ("a khu/_ug pa/_aM/", "ཨ་ཁུ། ཨུག་པ། ཨཾ།"),
    //
    // Rule 12:
    // Capitals are used to denote the following Sanskrit-based Tibetan characters: the long vowels – A, I, U, -I;
    // the anusvara – M; the visarga – H; the retroflex letters – T, Th, D, D+h, N, and Sh.
    ("mA duH phaT ", "མཱ་དུཿ་ཕཊ་"),
    //
    // Rule 13:
    // Capital R is used to indicate the full-form of ra when it is the top letter
    // of a non-standard Tibetan stack (equivalent to U+0F6A).
    ("R+na R+Ya R+ya ", "ཪྣ་ཪྻ་ཪྱ་"),
    //
    // Rule 14:
    // The full-formed ra in the standard Tibetan stacks—rnya, rla, and rwa—is transliterated as the lower-case “r”.
    ("rnya rla rwa ", "རྙ་རླ་རྭ་"),
    //
    // Rule 15:
    // Capital W, Y, and R are used to transliterate the full form of wa, ya, and ra respectively,
    // when they are in any position except the top-most.
    ("r+r r+R s+Wa r+Y ", "རྲ་རྼ་སྺ་རྻ་"),
    //
    // Rule 16:
    // In non-standard Tibetan stacks, the lower-case r, y, and w are used to represent the superscribed ra (ra mgo),
    // the subscribed ra (ra btags), the subscribed ya (ya btags), and the subscribed wa (wa zur) respectively.
    ("r+sha l+ra h+wa h+ya ", "རྴ་ལྲ་ཧྭ་ཧྱ་"),
];

#[test]
fn etu_rules_test() {
    let converter = crate::EwtsConverter::create();

    RULES_TST_DATA.iter().for_each(|td| {
        assert_eq!(converter.ewts_to_unicode(td.0), td.1);
    });
}
