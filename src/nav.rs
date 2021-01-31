use seed::Url;
use crate::i18n::LangExt;
use common::Lang;
use crate::ui::Page;

pub fn decode_url(mut url: Url) -> (Lang, Page) {
    let slice = url.remaining_hash_path_parts();
    log::info!("{:?}", slice);
    match slice.as_slice() {
        &["en"] => (Lang::En, Page::Home),
        &["fr"] => (Lang::Fr, Page::Home),
        &["en", "members"] => (Lang::En, Page::MppList),
        &["fr", "membres"] => (Lang::Fr, Page::MppList),
        &["en", "votes"] => (Lang::En, Page::VoteList),
        &["fr", "votes"] => (Lang::Fr, Page::VoteList),
        &["en", "members", riding] => (Lang::En, Page::Mpp(riding.to_string())),
        &["fr", "membres", riding] => (Lang::Fr, Page::Mpp(riding.to_string())),
        &["en", "votes", vote_id]=> {
            if let Ok(_) = vote_id.parse::<usize>() {
                (Lang::En, Page::NotFound(true))
            } else {
                (Lang::En, Page::Vote(vote_id.to_string()))
            }
        }
        &["fr", "votes", vote_id] => {
            if let Ok(_) = vote_id.parse::<usize>() {
                (Lang::Fr, Page::NotFound(true))
            } else {
                (Lang::Fr, Page::Vote(vote_id.to_string()))
            }
        }
        &["en", "faq"] => (Lang::En, Page::Faqs),
        &["fr", "faq"] => (Lang::Fr, Page::Faqs),
        &["en", ..] => (Lang::En, Page::NotFound(false)),
        &["fr", ..] => (Lang::Fr, Page::NotFound(false)),
        hash_path => {
            let mut full_path = vec!["en"];
            full_path.extend_from_slice(hash_path);
            let url = Url::current().set_hash_path(full_path);
            log::info!("{}", url);
            url.go_and_replace();
            decode_url(url) // guaranteed to be single iteration since there's a lang prefix now
        }
    }
}