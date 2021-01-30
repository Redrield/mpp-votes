use rust_embed::RustEmbed;
use i18n_embed::{
    fluent::{fluent_language_loader, FluentLanguageLoader},
    LanguageLoader
};
use i18n_embed_fl::fl;
use i18n_embed::unic_langid::LanguageIdentifier;
use lazy_static::lazy_static;

#[derive(RustEmbed)]
#[folder = "i18n/"]
struct Localizations;

lazy_static! {
    pub static ref LOADER: FluentLanguageLoader = {
        let loader = fluent_language_loader!();
        loader.load_languages(&Localizations, &[
            &LanguageIdentifier::from_bytes(b"en-CA").unwrap(),
            &LanguageIdentifier::from_bytes(b"fr-CA").unwrap()
        ]).unwrap();

        i18n_embed::select(&loader, &Localizations, &[
            LanguageIdentifier::from_bytes(b"fr-CA").unwrap(),
            LanguageIdentifier::from_bytes(b"en-CA").unwrap()
        ]).unwrap();
        loader
    };
}
