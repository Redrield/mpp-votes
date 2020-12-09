use seed::{*, prelude::*};
use crate::Msg;

pub fn content() -> Node<Msg> {
    div![C!["container"],
        section![C!["hero"],
            div![C!["hero-body"],
                div![C!["container"],
                    h1![C!["title has-text-centered is-1"], "ONVotes"],
                    div![C!["content is-medium"],
                        p!["This website holds records of all votes recorded in Ontario's Legislative Assembly's 42nd session in an accessible fashion.\
                        To find the voting history of a specific MPP, click the MPPs tab to search for them. To find a specific vote, click the Votes tab."]
                    ],
                    article![C!["message"],
                        div![C!["message-header"],
                            p!["The House is adjourned"]
                        ],
                        div![C!["message-body"], "The Ontario Parliament is adjourned until February 16, 2021."]
                    ]
                ]
            ]
        ]
    ]
}