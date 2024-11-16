use std::collections::HashMap;

use crate::dict::{Con, Sym, Vowel, CONSONANTS, CONS_LEN, SYM, SYM_LEN, VOWELS, VOWELS_LEN};

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum Token {
    Con(Con),
    Vowel(Vowel),
    Sym(Sym),
    Unknown(u8),
}

pub(crate) struct EwtsToUnicodeTokenizer<'a> {
    map: &'a EwtsToUnicodeTokenMap,
    ind: usize,
    src_len: usize,
    src_bytes: &'a [u8],
    result: Vec<Token>,
}

impl<'a> EwtsToUnicodeTokenizer<'a> {
    pub(crate) fn tokenize(map: &EwtsToUnicodeTokenMap, src: &str) -> Vec<Token> {
        let src_bytes = src.as_bytes();

        let tokenizer = EwtsToUnicodeTokenizer {
            map,
            ind: 0,
            src_len: src_bytes.len(),
            src_bytes,
            result: Vec::with_capacity(src_bytes.len()),
        };

        tokenizer.into_tokens()
    }

    fn into_tokens(mut self) -> Vec<Token> {
        while self.is_in_bounds() {
            let (tkn, len) = self.find_next();
            self.result.push(tkn);
            self.ind += len;
        }
        self.result
    }

    fn is_in_bounds(&self) -> bool {
        self.ind < self.src_len
    }

    fn find_next(&self) -> (Token, usize) {
        let curr_char = &self.src_bytes[self.ind];
        if let Some(first_ewts_char) = self.map.root.nodes.get(curr_char) {
            let mut i = 1;

            let mut last_valid: Option<(Token, usize)> = None;
            let mut process_ewts_char: Option<&EwtsChar> = Some(first_ewts_char);

            while let Some(ch) = &process_ewts_char {
                if let Some(t) = &ch.token {
                    last_valid = Some((t.clone(), i));
                }

                if self.ind + i == self.src_len {
                    break;
                }

                process_ewts_char = ch.nodes.get(&self.src_bytes[self.ind + i]);
                i += 1;
            }

            last_valid.unwrap_or((Token::Unknown(*curr_char), 1))
        } else {
            (Token::Unknown(*curr_char), 1)
        }
    }
}

pub(crate) struct EwtsToUnicodeTokenMap {
    root: EwtsChar,
}

impl EwtsToUnicodeTokenMap {
    pub(crate) fn create() -> Self {
        let mut map = EwtsToUnicodeTokenMap::new();
        map.fill_with(CONSONANTS.iter().map(|c| (Token::Con(c.0), c.1)).collect());
        map.fill_with(VOWELS.iter().map(|v| (Token::Vowel(v.0), v.1)).collect());
        map.fill_with(SYM.iter().map(|s| (Token::Sym(s.0), s.1)).collect());
        map
    }

    fn new() -> Self {
        EwtsToUnicodeTokenMap {
            root: EwtsChar {
                token: None,
                nodes: HashMap::with_capacity(CONS_LEN + VOWELS_LEN + SYM_LEN),
            },
        }
    }

    fn fill_with(&mut self, syllables: Vec<(Token, &'static str)>) {
        syllables.into_iter().for_each(|cons_tuple| {
            let ch_bytes = cons_tuple.1.as_bytes();
            let max_ind = ch_bytes.len() - 1;

            let mut parent_nodes = &mut self.root.nodes;

            for (i, c) in ch_bytes.iter().enumerate() {
                let entry = parent_nodes.entry(*c).or_insert_with(EwtsChar::new);
                if i == max_ind {
                    entry.token = Some(cons_tuple.0.clone());
                }
                parent_nodes = &mut entry.nodes;
            }
        });
    }
}

struct EwtsChar {
    token: Option<Token>,
    nodes: HashMap<u8, EwtsChar>,
}

impl EwtsChar {
    fn new() -> Self {
        EwtsChar {
            token: None,
            nodes: HashMap::with_capacity(2),
        }
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

    static TST_DATA: &[(&str, &[Token])] = &[
        ("dz+h", &[Token::Con(Con::DzPlusH)]),
        ("dz+gh", &[Token::Con(Con::Dz), Token::Unknown(43), Token::Con(Con::Gh)]),
        ("yry", &[Token::Con(Con::Y), Token::Con(Con::R), Token::Con(Con::Y)]),
        (
            "kg g+h dz+h brgyas-thh-T",
            &[
                Token::Con(Con::K),
                Token::Con(Con::G),
                Token::Sym(Sym::Space),
                Token::Con(Con::GPlusH),
                Token::Sym(Sym::Space),
                Token::Con(Con::DzPlusH),
                Token::Sym(Sym::Space),
                Token::Con(Con::B),
                Token::Con(Con::R),
                Token::Con(Con::G),
                Token::Con(Con::Y),
                Token::Vowel(Vowel::A),
                Token::Con(Con::S),
                Token::Con(Con::MinusTh),
                Token::Con(Con::H),
                Token::Unknown(45),
                Token::Con(Con::TBig),
            ],
        ),
        (
            "*///|",
            &[
                Token::Sym(Sym::Asterisk),
                Token::Sym(Sym::TwoSlashes),
                Token::Sym(Sym::Slash),
                Token::Sym(Sym::Bar),
            ],
        ),
    ];

    #[test]
    fn ewts_to_unicode_tokenizer_test() {
        let map = EwtsToUnicodeTokenMap::create();

        TST_DATA.iter().for_each(|td| {
            let tokens = EwtsToUnicodeTokenizer::tokenize(&map, td.0);
            println!("tst: src -- {:?} | result -- {:?}", td.0, tokens);
            assert_eq!(tokens, td.1);
        });
    }
}
