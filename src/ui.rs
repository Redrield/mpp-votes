use crate::{Msg, Model};
use seed::prelude::*;
use seed::*;
use std::borrow::Cow;
use common::Lang;
use crate::i18n::LangExt;

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
    Vote(String),
    Faqs,
    NotFound(bool),
}

impl Page {
    pub fn to_link(&self, lang: &Lang) -> String {
        match self {
            Page::Home => format!("#/{}", lang.to_href_prefix()),
            Page::MppList => match lang {
                Lang::En => "#/en/members".to_string(),
                Lang::Fr => "#/fr/membres".to_string(),
            },
            Page::Mpp(riding) => match lang {
                Lang::En => format!("#/en/members/{}", riding),
                Lang::Fr => format!("#/fr/membres/{}", riding)
            },
            Page::VoteList => format!("#/{}/votes", lang.to_href_prefix()),
            Page::Vote(vote_id) => format!("#/{}/votes/{}", lang.to_href_prefix(), vote_id),
            Page::Faqs => format!("#/{}/faq", lang.to_href_prefix()),
            Page::NotFound(_) => format!("#/{}/404", lang.to_href_prefix()),
        }
    }
}

impl Default for Page {
    fn default() -> Self {
        Page::Home
    }
}

pub fn navbar(model: &Model) -> Node<Msg> {
    let current_page = &model.current_page;
    let lang = &model.lang;
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
                a![C!["navbar-item", IF!(*current_page == Page::Home => "is-active")], attrs!{ At::Href => Page::Home.to_link(lang) }, fl!("navbar-item-home")],
                a![C!["navbar-item", IF!(*current_page == Page::MppList => "is-active")], attrs!{ At::Href => Page::MppList.to_link(lang) }, fl!("navbar-item-mpps")],
                a![C!["navbar-item", IF!(*current_page == Page::VoteList => "is-active")], attrs!{ At::Href => Page::VoteList.to_link(lang) }, fl!("navbar-item-votes")],
                a![C!["navbar-item", IF!(*current_page == Page::Faqs => "is-active")], attrs!{ At::Href => Page::Faqs.to_link(lang) }, fl!("navbar-item-faq")]
            ],
            div![C!["navbar-end"],
                IF!(*lang == Lang::En => a![C!["navbar-item"], "Français",
                    input_ev(Ev::Click, |_| Msg::ChangeLang(Lang::Fr))]),
                IF!(*lang == Lang::Fr => a![C!["navbar-item"], "English",
                    input_ev(Ev::Click, |_| Msg::ChangeLang(Lang::En))]),
                p![C!["navbar-item"], fl!("navbar-item-hansard", latest_date = latest_hansard)]
            ]
        ]
    ]
}

pub fn page(model: &Model) -> Node<Msg> {
    match &model.current_page {
        Page::Home => home::content(),
        Page::MppList => mpp::members_list(model),
        Page::Mpp(riding) => mpp::member_voting_record(riding, model),
        Page::Vote(idx) => vote::single_vote_record(idx, model),
        Page::VoteList => vote::vote_list(model),
        Page::Faqs => faqs::content(),
        Page::NotFound(from_vote) => error::error_404(*from_vote),
    }
}