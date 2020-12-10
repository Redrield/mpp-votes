use crate::{Model, Msg};
use seed::{*, prelude::*};
use super::mpp::member_card;
use sha::sha1::Sha1;
use sha::utils::{Digest, DigestExt};
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref BILL_RE: Regex = Regex::new(r"bill (?:pr)?\d+").unwrap();
}

fn topic_hash(topic: &str) -> String {
    Sha1::default().digest(topic.as_bytes()).to_hex()
}

pub fn vote_list(model: &Model) -> Node<Msg> {
    let cls = if model.displaying_search_error { "show" } else { "hide" };
    div![C!["container"],
        section![C!["hero"],
            div![C!["hero-body"],
                h1![C!["title has-text-centered is-1"], "Recorded Votes"],
                div![C!["content has-text-centered is-medium"],
                    p!["A list of all recorded votes in the current session. Click a vote to see the split."]
                ],
                div![C!["level"],
                    div![C!["level-item"],
                        div![C!["field has-addons"],
                            div![C!["control"],
                                input![C!["input"], attrs!{
                                    At::Type => "text",
                                    At::Placeholder => "Find a specific bill",
                                    At::Value => &model.query
                                }, input_ev(Ev::Input, Msg::QueryChanged), input_ev(Ev::Change, |_| Msg::Submit)]
                            ],
                            div![C!["control"],
                                button![C!["button", IF!(model.searching => "is-loading")], input_ev(Ev::Click, |_| Msg::Submit), "Search"]
                            ]
                        ]
                    ]
                ],
                article![C!["message", cls],
                    div![C!["message-body"], "Couldn't find anything matching that query."]
                ]
            ]
        ],
        section![C!["section"],
            div![C!["tile is-ancestor"],
                div![C!["tile is-parent is-vertical"],
                    model.display_divisions.as_ref().unwrap_or(&model.divisions).iter().map(|d| {
                        let digest = topic_hash(&d.topic);
                        div![C!["tile is-child box"],
                            a![attrs!{ At::Href => &format!("#/votes/{}", digest) }, style!{ St::Color => "inherit", St::TextDecoration => "inherit" },
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
    if let Some((i, vote)) = model.divisions.iter().enumerate().find(|(_, d)| Sha1::default().digest(d.topic.as_bytes()).to_hex() == hash) {
        div![C!["container"],
            section![C!["section"],
                div![C!["content has-text-centered is-medium"],
                    p![C!["title"], &format!("Vote {}", model.divisions.len() - i)],
                    p![C!["subtitle"], &vote.topic],
                    BILL_RE.captures(&vote.topic.to_lowercase()).map(|bill| {
                        a![attrs!{ At::Href => format!("https://www.ola.org/en/legislative-business/bills/parliament-42/session-1/{}", bill.get(0).unwrap().as_str().replace(" ", "-"))}, "Text of the bill"]
                    })
                ]
            ],
            section![C!["section"],
                div![C!["columns"],
                    div![C!["column"],
                        p![C!["title"], &format!("Yes ({})", vote.ayes.len())],
                        vote.ayes.iter().map(|m| member_card(m, "my-6"))
                    ],
                    div![C!["column"],
                        p![C!["title"], &format!("No ({})", vote.nays.len())],
                        vote.nays.iter().map(|m| member_card(m, "my-6"))
                    ]
                ]
            ]
        ]
    } else {
        super::error::error_404(false)
    }

}