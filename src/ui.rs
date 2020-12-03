use crate::{Msg, Model};
use seed::prelude::*;
use seed::*;
use std::borrow::Cow;

pub mod home;
pub mod mpp;
pub mod vote;
pub mod faqs;
pub mod footer;
pub mod error;

#[derive(Debug, Clone, PartialEq)]
pub enum Page {
    Home,
    MppList,
    Mpp(String),
    VoteList,
    Vote(usize),
    Faqs,
    NotFound,
}

impl Default for Page {
    fn default() -> Self {
        Page::Home
    }
}

impl From<Url> for Page {
    fn from(mut url: Url) -> Self {
        match url.remaining_hash_path_parts().as_slice() {
            &[] => Page::Home,
            &["members"] => Page::MppList,
            &["votes"] => Page::VoteList,
            &["members", riding] => Page::Mpp(riding.to_string()),
            &["votes", vote_id] => Page::Vote(vote_id.parse().unwrap()),
            &["faq"] => Page::Faqs,
            _ => Page::NotFound,
        }
    }
}

pub fn navbar(model: &Model) -> Node<Msg> {
    let current_page = &model.current_page;
    let navbar_active = model.navbar_active;
    // Date of the latest vote recorded in JSON
    let latest_hansard = model.divisions.first().map(|d| d.date.as_str()).unwrap_or("");
    nav![C!["navbar is-fixed-top has-shadow"], attrs!{ At::Custom(Cow::Borrowed("role")) => "navigation", At::AriaLabel => "Main Navigation" },
        div![C!["navbar-brand"],
            a![C!["navbar-burger burger", IF!(navbar_active => "is-active")], attrs!{ At::Custom(Cow::Borrowed("role")) => "button", At::AriaLabel => "menu", At::AriaExpanded => "false", At::Custom(Cow::Borrowed("data-target")) => "mainNavbar" },
                input_ev(Ev::Click, |_| Msg::NavbarClick),
                span![attrs!{ At::AriaHidden => "true" }],
                span![attrs!{ At::AriaHidden => "true" }],
                span![attrs!{ At::AriaHidden => "true" }],
            ]
        ],
        div![C!["navbar-menu", IF!(navbar_active => "is-active")], id!["mainNavbar"],
            div![C!["navbar-start"],
                a![C!["navbar-item", IF!(*current_page == Page::Home => "is-active")], attrs!{ At::Href => "#" }, "Home"],
                a![C!["navbar-item", IF!(*current_page == Page::MppList => "is-active")], attrs!{ At::Href => "#/members" }, "MPPs"],
                a![C!["navbar-item", IF!(*current_page == Page::VoteList => "is-active")], attrs!{ At::Href => "#/votes" }, "Votes"],
                a![C!["navbar-item", IF!(*current_page == Page::Faqs => "is-active")], attrs!{ At::Href => "#/faq" }, "FAQs"]
            ],
            div![C!["navbar-end"],
                p![C!["navbar-item"], &format!("Latest Hansard: {}", latest_hansard)]
            ]
        ]
    ]
}

pub fn page(model: &Model) -> Node<Msg> {
    match model.current_page {
        Page::Home => home::content(),
        Page::MppList => mpp::members_list(model),
        Page::Mpp(ref riding) => mpp::member_voting_record(riding, model),
        Page::Vote(idx) => vote::single_vote_record(idx, model),
        Page::VoteList => vote::vote_list(model),
        Page::Faqs => faqs::content(),
        _ => error::error_404(),
    }
}