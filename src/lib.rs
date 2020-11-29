// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};
use common::{Member, Division, Redirects};
use common::search::{SimSearch, SearchOptions};
use log::Level;
use std::sync::Mutex;
use std::borrow::Cow;
use crate::ui::Page;

mod startup;
mod util;
mod api;
// mod bg;
mod ui;

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(async {
        let data = api::ipfs_get("/ipns/k2k4r8ka63uxofwvctgqzl9xgz7h8c8sekmhdailvgf1pd9px5bogyxe").await;
        Msg::RedirectsFetched(serde_json::from_str(std::str::from_utf8(&data[..]).unwrap()).unwrap())
    });
    Model {
        members_search: SimSearch::new_with(SearchOptions::new().case_sensitive(false)),
        divisions: vec![],
        members: vec![],
        current_page: Page::Home,
        rdy: false,
    }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model {
    members_search: SimSearch<Member>,
    members: Vec<Member>,
    divisions: Vec<Division>,
    current_page: Page,
    rdy: bool,
}

// ------ ------
//    Update
// ------ ------

// `Msg` describes the different events you can modify state with.
pub enum Msg {
    RedirectsFetched(Redirects),
    MembersFetched(Vec<Member>),
    DivisionsFetched(Vec<Division>),
    MembersIndexed(SimSearch<Member>),
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::RedirectsFetched(redir) => {
            log::info!("Redirects fetched, getting resources.");
            let Redirects { members, divisions } = redir;
            orders.perform_cmd(async move {
                Msg::MembersFetched(startup::fetch_members(api::ipfs_get(&format!("/ipfs/{}", members)).await))
            });
            orders.perform_cmd(async move {
                Msg::DivisionsFetched(startup::fetch_divisions(api::ipfs_get(&format!("/ipfs/{}", divisions)).await))
            });
        }
        Msg::MembersFetched(members) => {
            model.members = members;
            let m = model.members.clone();
            orders.perform_cmd(async move {
                let mut search = SimSearch::new_with(SearchOptions::new().case_sensitive(false));
                for member in m {
                    let toks = &[member.full_name.as_str(), member.riding.as_str(), member.party.as_str()];
                    let member = member.clone();
                    search.insert_tokens(member, toks);
                }
                Msg::MembersIndexed(search)
            });
        }
        Msg::DivisionsFetched(divisions) => {
            model.divisions = divisions;
            //TODO: Index
        }
        Msg::MembersIndexed(search) => {
            model.members_search = search;
            model.rdy = true;
        },
    }
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Vec<Node<Msg>> {
    if model.rdy {
        nodes![
            ui::navbar(&model.current_page),
            ui::page(&model.current_page),
        ]
    } else {
        nodes![
            ui::navbar(&model.current_page),
            div![C!["container"],
                h2!["Loading..."],
            ]
        ]
    }
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    console_log::init_with_level(Level::Info);
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
