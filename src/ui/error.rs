use seed::{*, prelude::*};
use crate::Msg;

pub fn error_404(vote: bool) -> Node<Msg> {
    div![C!["container"],
        section![C!["hero"],
            div![C!["hero-body has-text-centered"],
                h1![C!["title is-1"], "Not Found"],
                h2![C!["subtitle"], "Whatever you were looking for could not be found."],
                IF!(vote =>
                    article![C!["message"],
                        div![C!["message-header"],
                            p!["Changed links"]
                        ],
                        div![C!["message-body"], "The links for votes were changed, as the previous format wasn't consistent after time had passed."]
                    ]
                )
            ]
        ]
    ]
}