use crate::dict::{Con, ConSpec, SUBSCRIPTS, SUPERSCRIPTS};
use crate::tokenizer::{Token, TokenType};
use std::collections::HashMap;

pub(crate) struct EwtsToUnicodeConverter<'a> {
    maps: &'a EwtsToUnicodeConverterMaps,
    ind: usize,
    tokens: &'a [Token],
    tokens_len: usize,
    prev_type: TokenType,
    prev_con_combined: bool,
    last_con_spec_is_plus: bool,
}

impl<'a> EwtsToUnicodeConverter<'a> {
    fn new(maps: &'a EwtsToUnicodeConverterMaps, tokens: &'a [Token]) -> Self {
        EwtsToUnicodeConverter {
            maps,
            ind: 0,
            tokens,
            tokens_len: tokens.len(),
            prev_type: TokenType::Sym,
            prev_con_combined: false,
            last_con_spec_is_plus: false,
        }
    }

    pub(crate) fn convert(maps: &EwtsToUnicodeConverterMaps, tokens: &'a [Token]) -> String {
        let converter = EwtsToUnicodeConverter::new(maps, tokens);
        converter.run()
    }

    fn run(mut self) -> String {
        let mut result = String::with_capacity(self.tokens_len * 3);

        //println!("tokens_len {} ", self.tokens_len);

        let a_chen_tib_str = Con::AChen.get().1;
        //println!("tokens=++++++ {:#?}", self.tokens);

        while self.is_in_bounds() {
            match self.tokens[self.ind] {
                Token::Con(c) => 'con: {
                    let tuple = c.get();

                    let mut curr_prev_con_combined = false;

                    if self.prev_type == TokenType::Con {
                        if c.is_a_chen() {
                            self.prev_type = TokenType::Vowel;
                            self.prev_con_combined = false;
                            break 'con;
                        }

                        if self.is_lower_form(c) {
                            result += tuple.2;
                            curr_prev_con_combined = true;
                        } else {
                            result += tuple.1;
                        }
                    } else if self.prev_type == TokenType::ConSpec && self.last_con_spec_is_plus {
                        result += tuple.2;
                        curr_prev_con_combined = true;
                    } else {
                        result += tuple.1;
                    }

                    self.prev_con_combined = curr_prev_con_combined;
                    self.prev_type = TokenType::Con;
                }
                Token::Vowel(v) => {
                    if self.prev_type == TokenType::Con
                        || (self.prev_type == TokenType::ConSpec && self.last_con_spec_is_plus)
                    {
                    } else {
                        result += a_chen_tib_str;
                    }
                    self.prev_type = TokenType::Vowel;
                    result += v.get().1;
                }
                Token::Sym(t) => {
                    self.prev_type = TokenType::Sym;
                    result += t.get().1;
                }
                Token::ConSpec(s) => {
                    //last_con_spec = Some(s);
                    if s == ConSpec::Plus {
                        self.last_con_spec_is_plus = true;
                    }
                    self.prev_type = TokenType::ConSpec;
                }
                Token::Final(f) => {
                    self.prev_type = TokenType::Final;
                    result += f.get().1;
                }
                Token::Unknown(u) => {
                    result.push(u as char);
                } //Token::NonTibetan(_) => todo!(),
            };

            self.ind += 1;
        }

        //println!("results_len {} ", result.len());
        result
    }

    fn is_lower_form(&self, con: Con) -> bool {
        if let Some(curr_con_as_sub) = self.maps.sup_sub.get(&con) {
            let prev_con = self.tokens[self.ind - 1].get_con().unwrap();

            if let Some(middle) = curr_con_as_sub.prevs.get(&prev_con) {
                if middle.is_finite && !self.prev_con_combined {
                    true
                } else {
                    let prev2_con = if self.ind < 2 {
                        None
                    } else {
                        self.tokens[self.ind - 2].get_con()
                    };

                    if let Some(prev2) = prev2_con {
                        middle.prevs.contains_key(&prev2)
                    } else {
                        middle.is_finite
                    }
                }
            } else {
                false
            }
        } else {
            false
        }
    }

    fn is_in_bounds(&self) -> bool {
        self.ind < self.tokens_len
    }
}

pub(crate) struct EwtsToUnicodeConverterMaps {
    sup_sub: HashMap<Con, SubSupCon>,
}

impl EwtsToUnicodeConverterMaps {
    pub(crate) fn create() -> Self {
        let mut sup_sub_map = HashMap::new();

        SUBSCRIPTS.iter().for_each(|s| {
            let sub = sup_sub_map.entry(s.0).or_insert(SubSupCon::new());

            s.1.iter().for_each(|middle_con| {
                sub.prevs.entry(*middle_con).or_insert(SubSupCon::new_finite());
            });

            s.2.iter().for_each(|(top_con, middle_con)| {
                let middle = sub.prevs.entry(*middle_con).or_insert(SubSupCon::new());
                middle.prevs.entry(*top_con).or_insert(SubSupCon::new_finite());
            });
        });

        SUPERSCRIPTS.iter().for_each(|s| {
            let sup = s.0;

            s.1.iter().for_each(|middle_con| {
                let lower = sup_sub_map.entry(*middle_con).or_insert(SubSupCon::new());
                lower.prevs.entry(sup).or_insert(SubSupCon::new_finite());
            });

            s.2.iter().for_each(|(middle_con, lower_con)| {
                let lower = sup_sub_map.entry(*lower_con).or_insert(SubSupCon::new());
                let middle = lower.prevs.entry(*middle_con).or_insert(SubSupCon::new());
                middle.prevs.entry(sup).or_insert(SubSupCon::new_finite());
            });
        });

        EwtsToUnicodeConverterMaps { sup_sub: sup_sub_map }
    }
}

struct SubSupCon {
    is_finite: bool,
    prevs: HashMap<Con, SubSupCon>,
}

impl SubSupCon {
    fn new() -> Self {
        SubSupCon {
            is_finite: false,
            prevs: HashMap::new(),
        }
    }

    fn new_finite() -> Self {
        let mut s = Self::new();
        s.is_finite = true;
        s
    }
}
