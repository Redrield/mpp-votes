use seed::{*, prelude::*};
use crate::Msg;

pub fn error_404(_vote: bool) -> Node<Msg> {
    div![C!["container"],
        section![C!["hero"],
            div![C!["hero-body has-text-centered"],
                h1![C!["title is-1"], fl!("error-404-title")],
                h2![C!["subtitle"], fl!("error-404-subtitle")]
            ]
        ]
    ]
}