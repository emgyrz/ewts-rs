use crate::{dict::CONSONANTS, tokenizer::Token};

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

    pub(crate) fn convert_outer(tokens: &'a [Token]) -> String {
        
        let converter = EwtsToUnicodeConverter::new(tokens);
        converter.convert(tokens)
    }

    pub(crate) fn convert(mut self, tokens: &'a [Token]) -> String {

        let mut result = String::with_capacity(self.tokens_len / 2);


        while self.is_in_bounds() {
            let ch: &str = match self.tokens[self.ind] {
                Token::Con(t) => {
                    let tuple = CONSONANTS.iter().find(|c| c.0 == t);
                    tuple.unwrap().2
                },
                _ => " ",
                //Token::Vowel(_) => todo!(),
                //Token::Sym(_) => todo!(),
                //Token::Final(_) => todo!(),
                //Token::ConSpec(_) => todo!(),
                //Token::Unknown(_) => todo!(),
            };

            result += ch;
            self.ind += 1;
        }

        //String::from_utf8(result).unwrap()
        result
    }

    fn is_in_bounds(&self) -> bool {
        self.ind < self.tokens_len
    }
}
