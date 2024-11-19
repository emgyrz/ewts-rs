use crate::dict::{Con, ConSpec, CONSONANTS, CON_SPEC, FINALS, SYM, VOWELS};
use crate::tokenizer::{Token,TokenType};

pub(crate) struct EwtsToUnicodeConverter<'a> {
    ind: usize,
    tokens: &'a [Token],
    tokens_len: usize,
}

impl<'a> EwtsToUnicodeConverter<'a> {
    fn new(tokens: &'a [Token]) -> Self {
        EwtsToUnicodeConverter {
            ind: 0,
            tokens,
            tokens_len: tokens.len(),
        }
    }

    pub(crate) fn convert(tokens: &'a [Token]) -> String {
        let converter = EwtsToUnicodeConverter::new(tokens);
        converter.run()
    }

    fn run(mut self) -> String {
        let mut result = String::with_capacity(self.tokens_len * 3);

        //println!("tokens_len {} ", self.tokens_len);

        let a_chen = CONSONANTS.iter().find(|c| c.0 == Con::AChen).unwrap();
        //println!("tokens=++++++ {:#?}", self.tokens);

        let mut prev_type = TokenType::Sym;
        let mut last_con_spec = None;

        while self.is_in_bounds() {
            match self.tokens[self.ind] {
                Token::Con(t) => 'con: {
                    let tuple = CONSONANTS.iter().find(|c| c.0 == t).unwrap();

                    if prev_type == TokenType::Con {
                        if t == Con::AChen {
                            prev_type = TokenType::Vowel;
                            break 'con;
                        } else {
                            result += tuple.3;
                        }
                    } else if prev_type == TokenType::ConSpec && last_con_spec.unwrap() == ConSpec::Plus {
                        result += tuple.3;
                    } else {
                        result += tuple.2;
                    }

                    prev_type = TokenType::Con;
                }
                Token::Vowel(v) => {
                    let tuple = VOWELS.iter().find(|t| t.0 == v).unwrap();
                    if prev_type != TokenType::Con {
                        result += a_chen.2;
                    }
                    result += tuple.2;
                    prev_type = TokenType::Vowel;
                }
                Token::Sym(t) => {
                    let tuple = SYM.iter().find(|s| s.0 == t).unwrap();
                    prev_type = TokenType::Sym;
                    result += tuple.2;
                }
                Token::Final(t) => {
                    let tuple = FINALS.iter().find(|f| f.0 == t).unwrap();
                    prev_type = TokenType::Final;
                    result += tuple.2;
                }
                Token::ConSpec(s) => {
                    match s {
                        ConSpec::Plus| ConSpec::Period => {
                            last_con_spec = Some(s);
                        },
                        _ => {
                            let tuple = CON_SPEC.iter().find(|t| t.0 == s).unwrap();
                            result += tuple.2;
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
