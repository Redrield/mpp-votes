use common::{Member, Division};
use crate::{Msg, Model};
use seed::prelude::*;
use seed::*;
use std::borrow::Cow;

pub mod home;
pub mod mpp;
pub mod vote;

#[derive(Debug, Clone, PartialEq)]
pub enum Page {
    Home,
    MppList,
    Mpp(String),
    VoteList,
    Vote(usize),
    NotFound,
}

impl Default for Page {
    fn default() -> Self {
        Page::Home
    }
}

impl Page {
    pub fn on_navbar(&self) -> bool {
        match self {
            Page::Home | Page::MppList | Page::VoteList => true,
            _ => false
        }
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
            _ => Page::NotFound,
        }
    }
}

pub fn navbar(current_page: &Page) -> Node<Msg> {
    nav![C!["navbar is-fixed-top has-shadow"], attrs!{ At::Custom(Cow::Borrowed("role")) => "navigation", At::AriaLabel => "Main Navigation" },
        div![C!["navbar-menu"], id!["mainNavbar"],
            div![C!["navbar-start"],
                a![C!["navbar-item", IF!(*current_page == Page::Home => "is-active")], attrs!{ At::Href => "#" }, "Home"],
                a![C!["navbar-item", IF!(*current_page == Page::MppList => "is-active")], attrs!{ At::Href => "#/members" }, "MPPs"],
                a![C!["navbar-item", IF!(*current_page == Page::VoteList => "is-active")], attrs!{ At::Href => "#/votes" }, "Votes"]
            ],
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
        ref page => todo!("{:?}", page)
    }
}