use rust_embed::RustEmbed;
use i18n_embed::{
    fluent::{fluent_language_loader, FluentLanguageLoader},
    LanguageLoader
};
use i18n_embed_fl::fl;
use i18n_embed::unic_langid::LanguageIdentifier;
use lazy_static::lazy_static;
use seed::Url;
use common::Lang;

#[derive(RustEmbed)]
#[folder = "i18n/"]
pub struct Localizations;

pub trait LangExt {
    fn to_language_identifier(&self) -> LanguageIdentifier;
    fn to_href_prefix(&self) -> &str;
}

impl LangExt for Lang {
    fn to_language_identifier(&self) -> LanguageIdentifier {
        match self {
            Lang::En => LanguageIdentifier::from_bytes(b"en-CA").unwrap(),
            Lang::Fr => LanguageIdentifier::from_bytes(b"fr-CA").unwrap()
        }
    }

    fn to_href_prefix(&self) -> &str {
        match self {
            Lang::En => "en",
            Lang::Fr => "fr",
        }
    }
}

lazy_static! {
    pub static ref LOADER: FluentLanguageLoader = {
        let loader = fluent_language_loader!();
        loader.load_languages(&Localizations, &[
            &LanguageIdentifier::from_bytes(b"en-CA").unwrap(),
            &LanguageIdentifier::from_bytes(b"fr-CA").unwrap()
        ]).unwrap();

        loader
    };
}
