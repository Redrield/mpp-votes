use seed::{*, prelude::*};
use crate::{Model, Msg};
use common::Member;

pub fn member_card(member: &Member, margin: impl ToClasses) -> Node<Msg> {
    div![C!["card", margin], style!{ St::MinWidth => "14rem", St::MinHeight=> "10rem" },
        header![C!["card-header", member.party.favourite_colour()],
            a![C!["has-text-white card-header-title"], attrs!{ At::Href => &format!("#/members/{}", member.riding) }, &member.full_name]
        ],
        div![C!["card-content"],
            div![C!["content is-small"],
                p![format!("Member for {}", member.riding)],
            ]
        ]
    ]
}

pub fn members_list(model: &Model) -> Node<Msg> {
    let cls = if model.displaying_search_error { "show" } else { "hide" };
    div![C!["container"],
        section![C!["hero"],
            div![C!["hero-body"],
                h1![C!["title has-text-centered is-1"], "Members of Provincial Parliament"],
                div![C!["content has-text-centered is-medium"],
                    p!["List of all currently sitting MPPs. Search by name, riding, or postal code."]
                ],
                div![C!["level"],
                    div![C!["level-item"],
                        div![C!["field has-addons"],
                            div![C!["control"],
                                input![C!["input"], attrs!{
                                    At::Type => "text",
                                    At::Placeholder => "Find your MPP",
                                    At::Value => &model.query
                                }, input_ev(Ev::Input, Msg::QueryChanged), input_ev(Ev::Change, |_| Msg::Submit)]
                            ],
                            div![C!["control"],
                                button![C!["button", IF!(model.searching => "is-loading")], input_ev(Ev::Click, |_| Msg::Submit), "Search"]
                            ]
                        ]
                    ],
                ],
                article![C!["message", cls],
                    div![C!["message-body"], "Couldn't find anything matching that query."]
                ]
            ]
        ],
        section![C!["section"],
            div![C!["level is-flex is-flex-direction-row"], style! { St::OverflowX => "scroll" },
                model.display_members.as_ref().unwrap_or(&model.members)
                    .iter().map(|m| member_card(m, "mx-3"))
            ]
        ]
    ]
}

pub fn member_voting_record(riding: &str, model: &Model) -> Node<Msg> {
    let member = model.members.iter().find(|m| m.riding == riding).unwrap();

    div![C!["container"],
        section![C!["section"],
           div![C!["content has-text-centered"],
               p![C!["title"], &member.full_name],
               p![C!["subtitle"],format!("MPP for {}", member.riding), br![], format!("Member of the {}", member.party.as_str())],
           ],
        ],
        section![C!["section"],
            p![C!["title has-text-centered"], "Voting Records"],
            div![C!["tile is-ancestor"],
                div![C!["tile is-parent is-vertical"],
                    model.divisions.iter().enumerate().map(|(i, d)| {
                        div![C!["tile is-child box"],
                            a![attrs!{ At::Href => &format!("#/votes/{}", i) }, style!{ St::Color => "inherit", St::TextDecoration => "inherit" },
                                p![&d.topic],
                                p![&d.date],
                                br![],
                                div![C!["content"],
                                    if d.ayes.contains(member) {
                                        p![format!("{} voted ", member.full_name), b!["Yes."]]
                                    } else if d.nays.contains(member) {
                                        p![format!("{} voted ", member.full_name), b!["No."]]
                                    } else {
                                        p!["No vote recorded."]
                                    }
                                ]
                            ]
                        ]
                    })
                ]
            ]
        ]
    ]
}