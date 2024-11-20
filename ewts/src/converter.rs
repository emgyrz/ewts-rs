use crate::dict::{Con, ConSpec};
use crate::tokenizer::{Token, TokenType};

pub(crate) struct EwtsToUnicodeConverter<'a> {
    maps: &'a EwtsToUnicodeConverterMaps,
    ind: usize,
    tokens: &'a [Token],
    tokens_len: usize,
}

impl<'a> EwtsToUnicodeConverter<'a> {
    fn new(maps: &'a EwtsToUnicodeConverterMaps, tokens: &'a [Token]) -> Self {
        EwtsToUnicodeConverter {
            maps,
            ind: 0,
            tokens,
            tokens_len: tokens.len(),
        }
    }

    pub(crate) fn convert(maps: &EwtsToUnicodeConverterMaps, tokens: &'a [Token]) -> String {
        let converter = EwtsToUnicodeConverter::new(maps, tokens);
        converter.run()
    }

    fn run(mut self) -> String {
        let mut result = String::with_capacity(self.tokens_len * 3);

        //println!("tokens_len {} ", self.tokens_len);

        let a_chen = Con::AChen.get().1;
        //println!("tokens=++++++ {:#?}", self.tokens);

        let mut prev_type = TokenType::Sym;
        let mut last_con_spec = None;

        while self.is_in_bounds() {
            match self.tokens[self.ind] {
                Token::Con(c) => 'con: {
                    let tuple = c.get();

                    if prev_type == TokenType::Con {
                        if c == Con::AChen {
                            prev_type = TokenType::Vowel;
                            break 'con;
                        } else {
                            // TODO: check sup and sub here
                            result += tuple.2;
                        }
                    } else if prev_type == TokenType::ConSpec && last_con_spec.unwrap() == ConSpec::Plus {
                        result += tuple.2;
                    } else {
                        result += tuple.1;
                    }

                    prev_type = TokenType::Con;
                }
                Token::Vowel(v) => {
                    if prev_type != TokenType::Con {
                        result += a_chen;
                    }
                    prev_type = TokenType::Vowel;
                    result += v.get().1;
                }
                Token::Sym(t) => {
                    prev_type = TokenType::Sym;
                    result += t.get().1;
                }
                Token::Final(f) => {
                    prev_type = TokenType::Final;
                    result += f.get().1;
                }
                Token::ConSpec(s) => {
                    match s {
                        ConSpec::Plus | ConSpec::Period => {
                            last_con_spec = Some(s);
                        }
                        _ => {
                            last_con_spec = Some(s);
                            result += s.get().1;
                        }
                    }
                    prev_type = TokenType::ConSpec;
                }
                Token::Unknown(u) => {
                    result.push(u as char);
                }
            };

            self.ind += 1;
        }

        //println!("results_len {} ", result.len());
        result
    }

    fn is_in_bounds(&self) -> bool {
        self.ind < self.tokens_len
    }
}

pub(crate) struct EwtsToUnicodeConverterMaps {}

impl EwtsToUnicodeConverterMaps {
    pub(crate) fn create() -> Self {
        EwtsToUnicodeConverterMaps {}
    }
}
