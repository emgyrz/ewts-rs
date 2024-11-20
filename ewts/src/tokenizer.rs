use crate::dict::{Con, ConSpec, Final, Sym, Vowel};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum Token {
    Con(Con),
    Vowel(Vowel),
    Sym(Sym),
    Final(Final),
    ConSpec(ConSpec),
    Unknown(u8),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum TokenType {
    Con,
    Vowel,
    ConSpec,

    // TODO: unite to Other?
    Sym,
    Final,
    Unknown,
}

pub(crate) struct EwtsToUnicodeTokenizer<'a> {
    map: &'a EwtsToUnicodeTokenMap,
    ind: usize,
    src_len: usize,
    src_bytes: &'a [u8],
}

impl<'a> EwtsToUnicodeTokenizer<'a> {
    pub(crate) fn tokenize(map: &EwtsToUnicodeTokenMap, src: &str) -> Vec<Token> {
        let src_bytes = src.as_bytes();

        let tokenizer = EwtsToUnicodeTokenizer {
            map,
            ind: 0,
            src_len: src_bytes.len(),
            src_bytes,
        };

        tokenizer.into_tokens()
    }

    fn into_tokens(mut self) -> Vec<Token> {
        let mut result = Vec::with_capacity(self.src_len);

        while self.is_in_bounds() {
            let (tkn, len) = self.find_next();
            result.push(tkn);
            self.ind += len;
        }

        result
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
        map.fill_with(Con::list().iter().map(|c| (Token::Con(*c), c.get().0)).collect());
        map.fill_with(Vowel::list().iter().map(|v| (Token::Vowel(*v), v.get().0)).collect());
        map.fill_with(Sym::list().iter().map(|s| (Token::Sym(*s), s.get().0)).collect());
        map.fill_with(Final::list().iter().map(|f| (Token::Final(*f), f.get().0)).collect());
        map.fill_with(
            ConSpec::list()
                .iter()
                .map(|s| (Token::ConSpec(*s), s.get().0))
                .collect(),
        );
        map
    }

    fn new() -> Self {
        EwtsToUnicodeTokenMap {
            root: EwtsChar {
                token: None,
                nodes: HashMap::with_capacity(81),
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
        (
            "dz+gh",
            &[Token::Con(Con::Dz), Token::ConSpec(ConSpec::Plus), Token::Con(Con::Gh)],
        ),
        ("yry", &[Token::Con(Con::Y), Token::Con(Con::R), Token::Con(Con::Y)]),
        (
            "rnga",
            &[Token::Con(Con::R), Token::Con(Con::Ng), Token::Con(Con::AChen)],
        ),
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
                Token::Con(Con::AChen),
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
