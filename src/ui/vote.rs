use crate::{Model, Msg};
use seed::{*, prelude::*};
use super::mpp::member_card;
use sha::sha1::Sha1;
use sha::utils::{Digest, DigestExt};
use regex::Regex;
use crate::ui::Page;
use common::Lang;
use lazy_static::lazy_static;

lazy_static! {
    static ref BILL_RE: Regex = Regex::new(r"bill (?:pr)?\d+").unwrap();
}

fn topic_hash(topic: &str) -> String {
    Sha1::default().digest(topic.as_bytes()).to_hex()
}

pub fn vote_list(model: &Model) -> Node<Msg> {
    let lang = &model.lang;

    let cls = if model.displaying_search_error { "show" } else { "hide" };
    div![C!["container"],
        section![C!["hero"],
            div![C!["hero-body"],
                h1![C!["title has-text-centered is-1"], fl!("vote-list-title")],
                div![C!["content has-text-centered is-medium"],
                    p![fl!("vote-list-subtitle")]
                ],
                div![C!["level"],
                    div![C!["level-item"],
                        div![C!["field has-addons"],
                            div![C!["control"],
                                input![C!["input"], attrs!{
                                    At::Type => "text",
                                    At::Placeholder => fl!("vote-list-search"),
                                    At::Value => &model.query
                                }, input_ev(Ev::Input, Msg::QueryChanged), input_ev(Ev::Change, |_| Msg::Submit)]
                            ],
                            div![C!["control"],
                                button![C!["button", IF!(model.searching => "is-loading")], input_ev(Ev::Click, |_| Msg::Submit), fl!("vote-list-search-button")]
                            ]
                        ]
                    ]
                ],
                article![C!["message", cls],
                    div![C!["message-body"], fl!("vote-list-not-found")]
                ]
            ]
        ],
        section![C!["section"],
            div![C!["tile is-ancestor"],
                div![C!["tile is-parent is-vertical"],
                    model.display_divisions.as_ref().unwrap_or(&model.divisions).iter().map(|d| {
                        let digest = topic_hash(&d.topic);
                        div![C!["tile is-child box"],
                            a![attrs!{ At::Href => Page::Vote(digest.clone()).to_link(lang) }, style!{ St::Color => "inherit", St::TextDecoration => "inherit" },
                                p![&d.topic],
                                p![&d.date]
                            ]
                        ]
                    })
                ]
            ]
        ]
    ]
}

pub fn single_vote_record(hash: &str, model: &Model) -> Node<Msg> {
    let lang = &model.lang;
    if let Some((i, vote)) = model.divisions.iter().enumerate().find(|(_, d)| Sha1::default().digest(d.topic.as_bytes()).to_hex() == hash) {
        let num = model.divisions.len() - i;
        div![C!["container"],
            section![C!["section"],
                div![C!["content has-text-centered is-medium"],
                    p![C!["title"], fl!("vote-title", vote_number = num)],
                    p![C!["subtitle"], &vote.topic],
                    BILL_RE.captures(&vote.topic.to_lowercase()).map(|bill| {
                        if *lang == Lang::En {
                            a![attrs!{ At::Href => &format!("https://www.ola.org/en/legislative-business/bills/parliament-42/session-1/{}", bill.get(0).unwrap().as_str().replace(" ", "-"))},
                                fl!("vote-text-link")]
                        } else {
                            a![attrs!{ At::Href => &format!("https://www.ola.org/fr/affaires-legislatives/projets-loi/legislature-42/session-1/{}", bill.get(0).unwrap().as_str().replace("bill", "projet loi").replace(" ", "-"))},
                                     fl!("vote-text-link")]
                        }
                    })
                ]
            ],
            section![C!["section"],
                div![C!["columns"],
                    div![C!["column"],
                        p![C!["title"], fl!("vote-ayes", ayes_num = vote.ayes.len())],
                        vote.ayes.iter().map(|m| member_card(lang, m, "my-6"))
                    ],
                    div![C!["column"],
                        p![C!["title"], fl!("vote-nays", nays_num = vote.nays.len())],
                        vote.nays.iter().map(|m| member_card(lang, m, "my-6"))
                    ]
                ]
            ]
        ]
    } else {
        super::error::error_404(false)
    }

}