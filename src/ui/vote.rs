use crate::{Model, Msg};
use seed::{*, prelude::*};
use super::mpp::member_card;

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
                        let i = model.divisions.iter().enumerate().find(|(_, d0)| d == *d0).unwrap().0;
                        div![C!["tile is-child box"],
                            a![attrs!{ At::Href => &format!("#/votes/{}", i) }, style!{ St::Color => "inherit", St::TextDecoration => "inherit" },
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

pub fn single_vote_record(idx: usize, model: &Model) -> Node<Msg> {
    let vote = &model.divisions[idx];

    div![C!["container"],
        section![C!["section"],
            div![C!["content has-text-centered"],
                p![C!["title"], &format!("Vote {}", model.divisions.len() - idx)],
                p![C!["subtitle"], &vote.topic]
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
}