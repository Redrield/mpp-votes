use seed::{*, prelude::*};
use crate::Msg;

pub fn content() -> Node<Msg> {
    div![C!["container"],
        section![C!["hero"],
            div![C!["hero-body"],
                div![C!["container"],
                    h1![C!["title has-text-centered is-1"], "ONVotes"],
                    div![C!["content is-medium"],
                        p![fl!("main-subtitle")]
                    ],
                    article![C!["message"],
                        div![C!["message-header"],
                            p![fl!("main-temp-adjourned")]
                        ],
                        div![C!["message-body"], fl!("main-temp-adjourned-subtitle")]
                    ]
                ]
            ]
        ]
    ]
}