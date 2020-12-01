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
use lazy_static::lazy_static;
use regex::Regex;

mod startup;
mod util;
mod api;
mod searcher;
// mod bg;
mod ui;

lazy_static! {
    static ref POSTAL_CODE_RE: Regex = Regex::new(r#"[a-zA-Z][0-9][a-zA-Z]\s[0-9][a-zA-Z][0-9]"#).unwrap();
}

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.subscribe(Msg::UrlChanged);
    orders.perform_cmd(async {
        let data = api::ipfs_get("/ipns/k2k4r8ka63uxofwvctgqzl9xgz7h8c8sekmhdailvgf1pd9px5bogyxe").await;
        Msg::RedirectsFetched(serde_json::from_str(&data).unwrap())
    });
    Model {
        members_search: SimSearch::new_with(SearchOptions::new().case_sensitive(false).threshold(0.6)),
        members: vec![],
        display_members: None,
        query: "".to_string(),
        divisions: vec![],
        current_page: Page::from(url),
        rdy: false,
        searching: false,
        navbar_active: false,
    }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
pub struct Model {
    members_search: SimSearch<Member>,
    members: Vec<Member>,
    display_members: Option<Vec<Member>>,
    query: String,
    divisions: Vec<Division>,
    current_page: Page,
    rdy: bool,
    searching: bool,
    navbar_active: bool,
}

// ------ ------
//    Update
// ------ ------

// `Msg` describes the different events you can modify state with.
pub enum Msg {
    RedirectsFetched(Redirects),
    MembersFetched(Vec<Member>),
    DivisionsFetched(Vec<Division>),
    UrlChanged(subs::UrlChanged),
    QueryChanged(String),
    SearchComplete(Vec<Member>),
    Submit,
    NavbarClick
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::NavbarClick => model.navbar_active = !model.navbar_active,
        Msg::Submit => {
            model.searching = true;
            let query = model.query.clone();
            let search = model.members_search.clone();
            orders.perform_cmd(async move {
                if POSTAL_CODE_RE.is_match(query.trim()) {
                    use gloo_timers::future::TimeoutFuture;
                    let _ = TimeoutFuture::new(50).await;
                    let result = api::lookup_postal_code(query.trim()).await;
                    Msg::SearchComplete(search.search(&result))
                } else {
                    cmds::timeout(50, move || Msg::SearchComplete(search.search(&query))).await
                }
            });
        },
        Msg::QueryChanged(query) => model.query = query,
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            let new_page = Page::from(url);
            web_sys::window().unwrap().scroll_to_with_x_and_y(0.0, 0.0);
            // match (&model.current_page, &new_page) {
            //     (&Page::MppList, &Page::Mpp(_)) | (&Page::Mpp(_), &Page::MppList) => {}
            //     _ => {
            //         model.display_members = None;
            //         model.query = "".to_string();
            //     }
            // }
            model.navbar_active = false;
            model.current_page = new_page;
        }
        Msg::RedirectsFetched(redir) => {
            log::info!("Redirects fetched, getting resources.");
            let Redirects { members, divisions } = redir;
            orders.perform_cmd(async move {
                Msg::MembersFetched(serde_json::from_str(&api::ipfs_get(&format!("/ipfs/{}", members)).await).unwrap())
            });
            orders.perform_cmd(async move {
                Msg::DivisionsFetched(serde_json::from_str(&api::ipfs_get(&format!("/ipfs/{}", divisions)).await).unwrap())
            });
        }
        Msg::MembersFetched(members) => {
            model.members = members;
            let m = model.members.clone();
            for member in m {
                let toks = &[member.full_name.as_str(), member.riding.as_str(), member.party.as_str()];
                let member = member.clone();
                model.members_search.insert_tokens(member, toks);
            }
        }
        Msg::DivisionsFetched(divisions) => {
            model.divisions = divisions;
            model.rdy = true;
            //TODO: Index
        }
        Msg::SearchComplete(result) => {
            log::info!("{:?}", result);
            if result.is_empty() {
                model.display_members = None;
            } else {
                model.display_members = Some(result);
            }
            model.searching = false;
        }
    }
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Vec<Node<Msg>> {
    if model.rdy {
        nodes![
            ui::navbar(model),
            ui::page(model),
        ]
    } else {
        nodes![
            ui::navbar(model),
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
    console_error_panic_hook::set_once();
    log::info!("Starting app...");
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
