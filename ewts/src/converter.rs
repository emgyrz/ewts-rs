use crate::dict::{Con, ConSpec, SUBSCRIPTS, SUPERSCRIPTS};
use crate::tokenizer::{Token, TokenType, TokenizeResult};
use std::collections::HashMap;

pub(crate) struct EwtsToUnicodeConverter<'a> {
    src: &'a str,
    maps: &'a EwtsToUnicodeConverterMaps,
    ind: usize,
    tokenize_result: &'a TokenizeResult,
    tokens_len: usize,
    prev_type: TokenType,
    prev_con_combined: bool,
    last_con_spec_is_plus: bool,
}

impl<'a> EwtsToUnicodeConverter<'a> {
    pub(crate) fn convert(maps: &EwtsToUnicodeConverterMaps, tokenize_result: &'a TokenizeResult, src: &str) -> String {
        let converter = EwtsToUnicodeConverter {
            src,
            maps,
            ind: 0,
            tokenize_result,
            tokens_len: tokenize_result.tokens.len(),
            prev_type: TokenType::Sym,
            prev_con_combined: false,
            last_con_spec_is_plus: false,
        };

        converter.run()
    }

    fn run(mut self) -> String {
        let mut result = String::with_capacity(self.tokens_len * 3);

        let a_chen_tib_str = Con::AChen.get().1;

        self.tokenize_result.tokens.iter().for_each(|tkn| {
            match tkn {
                Token::Con(c) => 'con: {
                    let tuple = c.get();

                    let mut curr_prev_con_combined = false;

                    if self.prev_type == TokenType::Con {
                        if c.is_a_chen() {
                            self.prev_type = TokenType::Vowel;
                            self.prev_con_combined = false;
                            break 'con;
                        }

                        if self.is_lower_form(*c) {
                            result.push_str(tuple.2);
                            curr_prev_con_combined = true;
                        } else {
                            result.push_str(tuple.1);
                        }
                    } else if self.prev_type == TokenType::ConSpec && self.last_con_spec_is_plus {
                        result.push_str(tuple.2);
                        curr_prev_con_combined = true;
                    } else {
                        result.push_str(tuple.1);
                    }

                    self.prev_con_combined = curr_prev_con_combined;
                    self.prev_type = TokenType::Con;
                }
                Token::Vowel(v) => {
                    if self.prev_type == TokenType::Con
                        || (self.prev_type == TokenType::ConSpec && self.last_con_spec_is_plus)
                    {
                    } else {
                        result.push_str(a_chen_tib_str);
                    }
                    self.prev_type = TokenType::Vowel;
                    result.push_str(v.get().1);
                }
                Token::Sym(t) => {
                    self.prev_type = TokenType::Sym;
                    result.push_str(t.get().1);
                }
                Token::ConSpec(s) => {
                    self.last_con_spec_is_plus = *s == ConSpec::Plus;
                    self.prev_type = TokenType::ConSpec;
                }
                Token::Final(f) => {
                    self.prev_type = TokenType::Final;
                    result.push_str(f.get().1);
                }
                Token::Unknown(u) => {
                    result.push(*u as char);
                }
                Token::NonTibetan(ind) => {
                    if let Some(range) = self.tokenize_result.non_tibetan.get(*ind as usize) {
                        result.push_str(&self.src[range.clone()]);
                    }
                }
            };
            self.ind += 1;
        });

        //println!("tokens_len {} -- results_len {} ", self.tokens_len, result.len());
        result
    }

    // TODO: rewrite this
    fn is_lower_form(&self, cur_con: Con) -> bool {
        if let Some(curr_con_as_sub) = self.maps.sup_sub.get(&cur_con) {
            // unwrap bc it runs only if self.prev_type == TokenType::Con
            let prev_con = self.tokenize_result.tokens[self.ind - 1].get_con().unwrap();

            if let Some(prev_con_as_sup) = curr_con_as_sub.prevs.get(&prev_con) {
                if prev_con_as_sup.is_finite && !self.prev_con_combined {
                    if let Some(next_con) = self.get_con_at(self.ind + 1) {
                        if next_con.is_a_chen() {
                            return true;
                        }

                        self.maps
                            .sup_sub
                            .get(&next_con)
                            .map(|bottom| bottom.can_be_under_two(&cur_con, &prev_con))
                            // TODO: add tests for both true or false
                            .unwrap_or(true)
                    } else {
                        true
                    }
                } else {
                    self.get_con_at(self.ind - 2)
                        .map(|prev2_con| prev_con_as_sup.can_be_under(&prev2_con))
                        // TODO: tests
                        .unwrap_or(prev_con_as_sup.is_finite)
                }
            } else {
                false
            }
        } else {
            false
        }
    }

    fn get_con_at(&self, ind: usize) -> Option<Con> {
        self.tokenize_result.tokens.get(ind).map(|tkn| tkn.get_con())?
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

    fn can_be_under(&self, con: &Con) -> bool {
        self.prevs.get(con).map_or(false, |con_as_sub| con_as_sub.is_finite)
    }

    fn can_be_under_two(&self, con_middle: &Con, con_top: &Con) -> bool {
        self.prevs
            .get(con_middle)
            .map(|middle| middle.can_be_under(con_top))
            .unwrap_or(false)
    }
}
