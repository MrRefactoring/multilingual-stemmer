//! This library provides rust implementations for some stemmer algorithms
//! written in the [snowball language](https://snowballstem.org/).
//!
//!
//! All algorithms expect the input to already be lowercased.
//!
//! # Usage
//! ```toml
//! [dependencies]
//! rust-stemmers = "^1.0"
//! ```
//!
//! ```rust
//! extern crate rust_stemmers;
//!
//! use rust_stemmers::{Languages, Stemmer};
//!
//! fn main() {
//!    let en_stemmer = Stemmer::new(Languages::English);
//!    assert_eq!(en_stemmer.stem("fruitlessly"), "fruitless");
//! }
//! ```
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate wasm_bindgen;

mod snowball;

use snowball::SnowballEnv;
use snowball::algorithms;

use wasm_bindgen::prelude::*;

/// Enum of all supported languages.
/// Check the [Snowball-Website](https://snowballstem.org/) for details.
#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
pub enum Languages {
    Arabic,
    Basque,
    Catalan,
    Danish,
    Dutch,
    English,
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
    Portuguese,
    Romanian,
    Russian,
    Spanish,
    Swedish,
    Tamil,
    Turkish
}

/// Wrapps a usable interface around the actual stemmer implementation
#[wasm_bindgen]
pub struct Stemmer {
    stemmer: fn(&mut SnowballEnv) -> bool,
}

#[wasm_bindgen]
impl Stemmer {
    /// Create a new stemmer from an algorithm
    #[wasm_bindgen(constructor)]
    pub fn new(language: Languages) -> Self {
        match language {
            Languages::Arabic => Stemmer { stemmer: algorithms::arabic::stem },
            Languages::Basque => Stemmer { stemmer: algorithms::basque::stem },
            Languages::Catalan => Stemmer { stemmer: algorithms::catalan::stem },
            Languages::Danish => Stemmer { stemmer: algorithms::danish::stem },
            Languages::Dutch => Stemmer { stemmer: algorithms::dutch::stem },
            Languages::English => Stemmer { stemmer: algorithms::english::stem },
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
            Languages::Portuguese => Stemmer { stemmer: algorithms::portuguese::stem },
            Languages::Romanian => Stemmer { stemmer: algorithms::romanian::stem },
            Languages::Russian => Stemmer { stemmer: algorithms::russian::stem },
            Languages::Spanish => Stemmer { stemmer: algorithms::spanish::stem },
            Languages::Swedish => Stemmer { stemmer: algorithms::swedish::stem },
            Languages::Tamil => Stemmer { stemmer: algorithms::tamil::stem },
            Languages::Turkish => Stemmer { stemmer: algorithms::turkish::stem },
        }
    }

    /// Stem a single word
    /// Please note, that the input is expected to be all lowercase (if that is applicable).
    pub fn stem(&self, input: &str) -> String {
        let mut env = SnowballEnv::create(input);
        (self.stemmer)(&mut env);
        env.get_current().into()
    }
}
