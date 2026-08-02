mod snowball;

use crate::snowball::SnowballEnv;
use crate::snowball::algorithms;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Languages {
    Arabic,
    Armenian,
    Basque,
    Catalan,
    Czech,
    Danish,
    Dutch,
    English,
    Esperanto,
    Estonian,
    Finnish,
    French,
    German,
    Greek,
    Hindi,
    Hungarian,
    Indonesian,
    Irish,
    Italian,
    Lithuanian,
    Nepali,
    Norwegian,
    Persian,
    Polish,
    Portuguese,
    Romanian,
    Russian,
    Serbian,
    Sesotho,
    Spanish,
    Swedish,
    Tamil,
    Turkish,
    Yiddish,
}

#[wasm_bindgen]
pub struct Stemmer {
    stemmer: fn(&mut SnowballEnv) -> bool,
}

#[wasm_bindgen]
impl Stemmer {
    #[wasm_bindgen(constructor)]
    pub fn new(language: Languages) -> Self {
        match language {
            Languages::Arabic => Stemmer { stemmer: algorithms::arabic::stem },
            Languages::Armenian => Stemmer { stemmer: algorithms::armenian::stem },
            Languages::Basque => Stemmer { stemmer: algorithms::basque::stem },
            Languages::Catalan => Stemmer { stemmer: algorithms::catalan::stem },
            Languages::Czech => Stemmer { stemmer: algorithms::czech::stem },
            Languages::Danish => Stemmer { stemmer: algorithms::danish::stem },
            Languages::Dutch => Stemmer { stemmer: algorithms::dutch::stem },
            Languages::English => Stemmer { stemmer: algorithms::english::stem },
            Languages::Esperanto => Stemmer { stemmer: algorithms::esperanto::stem },
            Languages::Estonian => Stemmer { stemmer: algorithms::estonian::stem },
            Languages::Finnish => Stemmer { stemmer: algorithms::finnish::stem },
            Languages::French => Stemmer { stemmer: algorithms::french::stem },
            Languages::German => Stemmer { stemmer: algorithms::german::stem },
            Languages::Greek => Stemmer { stemmer: algorithms::greek::stem },
            Languages::Hindi => Stemmer { stemmer: algorithms::hindi::stem },
            Languages::Hungarian => Stemmer { stemmer: algorithms::hungarian::stem },
            Languages::Indonesian => Stemmer { stemmer: algorithms::indonesian::stem },
            Languages::Irish => Stemmer { stemmer: algorithms::irish::stem },
            Languages::Italian => Stemmer { stemmer: algorithms::italian::stem },
            Languages::Lithuanian => Stemmer { stemmer: algorithms::lithuanian::stem },
            Languages::Nepali => Stemmer { stemmer: algorithms::nepali::stem },
            Languages::Norwegian => Stemmer { stemmer: algorithms::norwegian::stem },
            Languages::Persian => Stemmer { stemmer: algorithms::persian::stem },
            Languages::Polish => Stemmer { stemmer: algorithms::polish::stem },
            Languages::Portuguese => Stemmer { stemmer: algorithms::portuguese::stem },
            Languages::Romanian => Stemmer { stemmer: algorithms::romanian::stem },
            Languages::Russian => Stemmer { stemmer: algorithms::russian::stem },
            Languages::Serbian => Stemmer { stemmer: algorithms::serbian::stem },
            Languages::Sesotho => Stemmer { stemmer: algorithms::sesotho::stem },
            Languages::Spanish => Stemmer { stemmer: algorithms::spanish::stem },
            Languages::Swedish => Stemmer { stemmer: algorithms::swedish::stem },
            Languages::Tamil => Stemmer { stemmer: algorithms::tamil::stem },
            Languages::Turkish => Stemmer { stemmer: algorithms::turkish::stem },
            Languages::Yiddish => Stemmer { stemmer: algorithms::yiddish::stem },
        }
    }

    pub fn stem(&self, input: &str) -> String {
        let mut env = SnowballEnv::create(input);
        (self.stemmer)(&mut env);
        env.get_current().into()
    }
}
