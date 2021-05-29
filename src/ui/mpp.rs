use seed::{*, prelude::*};
use crate::{Model, Msg};
use common::{Member, Lang};
use crate::ui::Page;
use sha::sha1::Sha1;
use sha::utils::{Digest, DigestExt};

fn topic_hash(topic: &str) -> String {
    Sha1::default().digest(topic.as_bytes()).to_hex()
}

pub fn member_card(lang: &Lang, member: &Member, margin: impl ToClasses) -> Node<Msg> {
    div![C!["card", margin], style!{ St::MinWidth => "14rem", St::MinHeight=> "10rem" },
        header![C!["card-header", member.party.favourite_colour()],
            a![C!["has-text-white card-header-title"], attrs!{ At::Href => Page::Mpp(member.riding.clone()).to_link(lang) }, &member.full_name]
        ],
        div![C!["card-content"],
            div![C!["content is-small"],
                p![fl!("member-mpp-for", riding = member.riding.as_str())]
            ]
        ]
    ]
}

pub fn members_list(model: &Model) -> Node<Msg> {
    let lang = &model.lang;
    let cls = if model.displaying_search_error { "show" } else { "hide" };
    div![C!["container"],
        section![C!["hero"],
            div![C!["hero-body"],
                h1![C!["title has-text-centered is-1"], fl!("member-list-title")],
                div![C!["content has-text-centered is-medium"],
                    p![fl!("member-list-subtitle")]
                ],
                div![C!["level"],
                    div![C!["level-item"],
                        div![C!["field has-addons"],
                            div![C!["control"],
                                input![C!["input"], attrs!{
                                    At::Type => "text",
                                    At::Placeholder => fl!("member-search-placeholder"),
                                    At::Value => &model.query
                                }, input_ev(Ev::Input, Msg::QueryChanged), input_ev(Ev::Change, |_| Msg::Submit)]
                            ],
                            div![C!["control"],
                                button![C!["button", IF!(model.searching => "is-loading")], input_ev(Ev::Click, |_| Msg::Submit), fl!("member-search-button")]
                            ]
                        ]
                    ],
                ],
                article![C!["message", cls],
                    div![C!["message-body"], fl!("member-search-not-found")]
                ]
            ]
        ],
        section![C!["section"],
            div![C!["level is-flex is-flex-direction-row"], style! { St::OverflowX => "scroll" },
                model.display_members.as_ref().unwrap_or(&model.members)
                    .iter().map(|m| member_card(lang, m, "mx-3"))
            ]
        ]
    ]
}

pub fn member_voting_record(riding: &str, model: &Model) -> Node<Msg> {
    let member = model.members.iter().find(|m| m.riding == riding).unwrap();
    let lang = &model.lang;

    div![C!["container"],
        section![C!["section"],
           div![C!["content has-text-centered"],
               p![C!["title"], &member.full_name],
               p![C!["subtitle"], fl!("member-mpp-for", riding = member.riding.as_str()), br![], fl!("member-member-of", party = member.party.as_str(lang))],
           ],
        ],
        section![C!["section"],
            p![C!["title has-text-centered"], fl!("member-voting-records")],
            div![C!["tile is-ancestor"],
                div![C!["tile is-parent is-vertical"],
                    model.divisions.iter().map(|d| {
                        let digest = topic_hash(&d.topic);
                        div![C!["tile is-child box"],
                            a![attrs!{ At::Href => &format!("#/votes/{}", digest) }, style!{ St::Color => "inherit", St::TextDecoration => "inherit" },
                                p![&d.topic],
                                p![&d.date],
                                br![],
                                div![C!["content"],
                                    if d.ayes.contains(member) {
                                        p![fl!("member-vote-record", name = member.full_name.as_str()), " ", b![fl!("member-vote-yes")]]
                                    } else if d.nays.contains(member) {
                                        p![fl!("member-vote-record", name = member.full_name.as_str()), " ", b![fl!("member-vote-no")]]
                                    } else {
                                        p![fl!("member-vote-neither")]
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