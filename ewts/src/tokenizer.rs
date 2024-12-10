use crate::dict::{Con, ConSpec, Final, Sym, Vowel};
use std::{char, collections::HashMap, ops::Range};

static SYM_SLASH: u8 = b'\\';
static SYM_BRACKET_L: u8 = b'[';
static SYM_BRACKET_R: u8 = b']';
static SYM_U: u8 = b'u';
static SYM_U_BIG: u8 = b'U';

#[cfg_attr(test, derive(Debug))]
#[derive(PartialEq, Eq, Clone)]
pub(crate) enum Token {
    Con(Con),
    Vowel(Vowel),
    Sym(Sym),
    Final(Final),
    ConSpec(ConSpec),
    NonTibetanStrRange(u8),
    NonTibetanCharIndex(u8),
    Unknown(u8),
}

impl Token {
    pub(crate) fn get_con(&self) -> Option<Con> {
        match self {
            Token::Con(c) => Some(*c),
            _ => None,
        }
    }
}

#[derive(PartialEq, Eq)]
pub(crate) enum TokenType {
    Con,
    Vowel,
    ConSpec,
    Other,
}

pub(crate) struct EwtsToUnicodeTokenizer<'a> {
    map: &'a EwtsToUnicodeTokenMap,
    ind: usize,
    src_len: usize,
    src_bytes: &'a [u8],
    result: TokenizeResult,
}

impl EwtsToUnicodeTokenizer<'_> {
    pub(crate) fn tokenize(map: &EwtsToUnicodeTokenMap, src: &str) -> TokenizeResult {
        let src_bytes = src.as_bytes();
        let src_len = src_bytes.len();

        let tokenizer = EwtsToUnicodeTokenizer {
            map,
            ind: 0,
            src_len,
            src_bytes,
            result: TokenizeResult::new(src_len),
        };

        tokenizer.run()
    }

    fn run(mut self) -> TokenizeResult {
        while self.is_in_bounds() {
            let (tkn, len) = self.find_next();
            self.result.tokens.push(tkn);
            self.ind += len;
        }

        self.result
    }

    fn is_in_bounds(&self) -> bool {
        self.ind < self.src_len
    }

    fn find_next(&mut self) -> (Token, usize) {
        let curr_char = self.src_bytes[self.ind];
        if let Some(first_ewts_char) = self.map.root.nodes.get(&curr_char) {
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

            last_valid.unwrap_or((Token::Unknown(curr_char), 1))
        } else if curr_char == SYM_SLASH {
            self.get_non_tibetan_char()
        } else if curr_char == SYM_BRACKET_L {
            self.get_non_tibetan_str()
        } else {
            (Token::Unknown(curr_char), 1)
        }
    }

    fn get_non_tibetan_str(&mut self) -> (Token, usize) {
        let mut len = 1;

        while self.ind + len < self.src_len && self.src_bytes[self.ind + len] != SYM_BRACKET_R {
            len += 1;
        }

        self.result.non_tibetan_str_ranges.push(Range {
            start: self.ind + 1,
            end: self.ind + len,
        });

        (Token::NonTibetanStrRange(self.result.str_ranges_last_ind()), len + 1)
    }

    fn get_non_tibetan_char(&mut self) -> (Token, usize) {
        if self.ind + 1 >= self.src_len {
            return (Token::Unknown(self.src_bytes[self.ind]), 1);
        }

        let char_after_slash = self.src_bytes[self.ind + 1];

        let is_u = char_after_slash == SYM_U;
        let is_u_big = char_after_slash == SYM_U_BIG;

        if is_u || is_u_big {
            let len = if is_u { 4 } else { 8 };

            self.get_next_n_bytes(self.ind + 2, len)
                .map(|unicode_char_bytes| String::from_utf8_lossy(unicode_char_bytes))
                .and_then(|s| u32::from_str_radix(s.as_ref(), 16).ok())
                .and_then(char::from_u32)
                .map_or_else(
                    || (Token::Unknown(self.src_bytes[self.ind]), 1),
                    |ch| {
                        self.result.non_tibetan_chars.push(ch);
                        (Token::NonTibetanCharIndex(self.result.chars_last_ind()), 2 + len)
                    },
                )
        } else {
            self.result.non_tibetan_chars.push(char_after_slash as char);
            (Token::NonTibetanCharIndex(self.result.chars_last_ind()), 2)
        }
    }

    fn get_next_n_bytes(&self, from: usize, n: usize) -> Option<&[u8]> {
        if from >= self.src_len || from + n > self.src_len {
            return None;
        }

        Some(&self.src_bytes[from..(from + n)])
    }
}

pub(crate) struct TokenizeResult {
    pub(crate) tokens: Vec<Token>,
    pub(crate) non_tibetan_str_ranges: Vec<Range<usize>>,
    pub(crate) non_tibetan_chars: Vec<char>,
}

impl TokenizeResult {
    fn new(tokens_capacity: usize) -> Self {
        TokenizeResult {
            tokens: Vec::with_capacity(tokens_capacity),
            non_tibetan_str_ranges: Vec::with_capacity(3),
            non_tibetan_chars: Vec::with_capacity(3),
        }
    }

    fn str_ranges_last_ind(&self) -> u8 {
        self.non_tibetan_str_ranges.len() as u8 - 1
    }

    fn chars_last_ind(&self) -> u8 {
        self.non_tibetan_chars.len() as u8 - 1
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
    fn etu_tokenizer_test() {
        let map = EwtsToUnicodeTokenMap::create();

        TST_DATA.iter().for_each(|td| {
            let result = EwtsToUnicodeTokenizer::tokenize(&map, td.0);
            assert_eq!(result.tokens, td.1);
        });
    }

    #[test]
    fn etu_tokenizer_non_tibetan_test() {
        let data = [
            (
                "k [alphabet]h ",
                &[
                    Token::Con(Con::K),
                    Token::Sym(Sym::Space),
                    Token::NonTibetanStrRange(0),
                    Token::Con(Con::H),
                    Token::Sym(Sym::Space),
                ] as &[Token],
            ),
            ("k[not_finished", &[Token::Con(Con::K), Token::NonTibetanStrRange(0)]),
            ("\\u0f40", &[Token::NonTibetanCharIndex(0)]),
            ("\\u0f4", &[Token::Unknown(92), Token::Vowel(Vowel::U), Token::Sym(Sym::Zero), Token::Con(Con::F), Token::Sym(Sym::Four)]),
        ];

        let map = EwtsToUnicodeTokenMap::create();

        data.iter().for_each(|td| {
            let result = EwtsToUnicodeTokenizer::tokenize(&map, td.0);
            assert_eq!(result.tokens, td.1);
        });
    }
}
