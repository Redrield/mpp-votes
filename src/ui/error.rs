use seed::{*, prelude::*};
use crate::Msg;

pub fn error_404() -> Node<Msg> {
    div![C!["container"],
        section![C!["hero"],
            div![C!["hero-body has-text-centered"],
                h1![C!["title is-1"], "Not Found"],
                h2![C!["subtitle"], "Whatever you were looking for could not be found."]
            ]
        ]
    ]
}