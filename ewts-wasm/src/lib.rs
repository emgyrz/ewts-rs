extern crate wasm_bindgen;
use ewts::EwtsConverter;
use wasm_bindgen::prelude::*;


#[wasm_bindgen(js_name = EwtsConverter)]
pub struct EwtsConverterJsWrapper {
    converter: EwtsConverter,
}

#[wasm_bindgen(js_class = EwtsConverter)]
impl EwtsConverterJsWrapper {
    #[allow(clippy::new_without_default)]
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        EwtsConverterJsWrapper {
            converter: EwtsConverter::create(),
        }
    }

    #[wasm_bindgen(js_name = ewtsToUnicode)]
    pub fn ewts_to_unicode(&self, src: &str) -> String {
        self.converter.ewts_to_unicode(src)
    }
}
