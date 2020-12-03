// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};
use common::{Member, Division};
use common::search::{SimSearch, SearchOptions};
use log::Level;
use crate::ui::Page;
use lazy_static::lazy_static;
use regex::Regex;

mod api;
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
        let data = Request::new("/data/members")
            .method(Method::Get)
            .fetch().await.unwrap()
            .json::<Vec<Member>>()
            .await.unwrap();
        Msg::MembersFetched(data)
    });
    orders.perform_cmd(async {
        let data = Request::new("/data/divisions")
            .method(Method::Get)
            .fetch().await.unwrap()
            .json::<Vec<Division>>()
            .await.unwrap();
        Msg::DivisionsFetched(data)
    });

    Model {
        members_search: SimSearch::new_with(SearchOptions::new().case_sensitive(false).threshold(0.6)),
        members: vec![],
        display_members: None,
        query: "".to_string(),
        divisions: vec![],
        display_divisions: None,
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
    display_divisions: Option<Vec<Division>>,
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
    MembersFetched(Vec<Member>),
    DivisionsFetched(Vec<Division>),
    UrlChanged(subs::UrlChanged),
    QueryChanged(String),
    MemberSearchComplete(Vec<Member>),
    DivisionSearchComplete(Vec<Division>),
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

            match model.current_page {
                Page::MppList => {
                    let search = model.members_search.clone();
                    orders.perform_cmd(async move {
                        if POSTAL_CODE_RE.is_match(query.trim()) {
                            use gloo_timers::future::TimeoutFuture;
                            let _ = TimeoutFuture::new(50).await;
                            let result = api::lookup_postal_code(query.trim()).await;
                            Msg::MemberSearchComplete(search.search(&result))
                        } else {
                            cmds::timeout(50, move || Msg::MemberSearchComplete(search.search(&query))).await
                        }
                    });
                }
                Page::VoteList => {
                    orders.perform_cmd(async move {
                        use gloo_timers::future::TimeoutFuture;
                        let _ = TimeoutFuture::new(50).await;
                        let response = Request::new(&format!("/api/search?query={}", query))
                            .method(Method::Get)
                            .fetch()
                            .await.unwrap()
                            .json::<Vec<Division>>()
                            .await.unwrap();
                        Msg::DivisionSearchComplete(response)
                    });
                }
                _ => unreachable!()
            }
        },
        Msg::QueryChanged(query) => model.query = query,
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            let new_page = Page::from(url);
            web_sys::window().unwrap().scroll_to_with_x_and_y(0.0, 0.0);
            match (&model.current_page, &new_page) {
                (&Page::MppList, &Page::Mpp(_)) | (&Page::Mpp(_), &Page::MppList) => {},
                (&Page::VoteList, &Page::Vote(_)) | (&Page::Vote(_), &Page::VoteList) => {},
                _ => {
                    model.display_members = None;
                    model.query = "".to_string();
                }
            }
            model.navbar_active = false;
            model.current_page = new_page;
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
        }
        Msg::MemberSearchComplete(result) => {
            log::info!("{:?}", result);
            if result.is_empty() {
                model.display_members = None;
            } else {
                model.display_members = Some(result);
            }
            model.searching = false;
        }
        Msg::DivisionSearchComplete(result) => {
            if result.is_empty() {
                model.display_divisions = None;
            } else {
                model.display_divisions = Some(result);
            }
            model.searching = false;
        }
    }
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    if model.rdy {
        div![C![IF!(model.current_page == Page::Home => "root")],
            ui::navbar(model),
            ui::page(model),
            ui::footer::content(),
        ]
    } else {
        div![C!["root"],
            ui::navbar(model),
            div![C!["container"],
                h2!["Loading..."],
            ],
            ui::footer::content(),
        ]
    }
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    console_log::init_with_level(Level::Info).unwrap();
    log::info!("Starting app...");
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
