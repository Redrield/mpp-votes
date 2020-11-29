use common::{Member, Division};
use crate::Msg;
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
    Mpp(Member),
    VoteList,
    Vote(Division),
}

impl Page {
    pub fn on_navbar(&self) -> bool {
        match self {
            Page::Home | Page::MppList | Page::VoteList => true,
            _ => false
        }
    }
}

pub fn navbar(current_page: &Page) -> Node<Msg> {
    nav![C!["navbar has-shadow"], attrs!{ At::Custom(Cow::Borrowed("role")) => "navigation", At::AriaLabel => "Main Navigation" },
        div![C!["navbar-menu"], id!["mainNavbar"],
            div![C!["navbar-start"],
                a![C!["navbar-item", IF!(*current_page == Page::Home => "is-active")], attrs!{ At::Href => "#" }, "Home"],
                a![C!["navbar-item", IF!(*current_page == Page::MppList => "is-active")], attrs!{ At::Href => "#/members" }, "MPPs"],
                a![C!["navbar-item", IF!(*current_page == Page::VoteList => "is-active")], attrs!{ At::Href => "#/votes" }, "Votes"]
            ],
        ]
    ]
}

pub fn page(current_page: &Page) -> Node<Msg> {
    match *current_page {
        Page::Home => home::content(),
        _ => todo!()
    }
}