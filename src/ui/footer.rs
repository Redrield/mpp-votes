use seed::{*, prelude::*};
use crate::Msg;

pub fn content() -> Node<Msg> {
    footer![C!["footer"],
        div![C!["level"],
            div![C!["level-left"],
                div![C!["level-item"],
                    p![fl!("footer-foreword"), br![], fl!("footer-next"), " ",
                        a![attrs!{ At::Href => "https://ola.org" }, fl!("footer-assembly")]
                    ],
                ],
            ],
            div![C!["level-right"],
                div![C!["level-item"],
                    a![attrs!{ At::Href => "https://github.com/Redrield/onvotes" }, style!{ St::Color => "inherit", St::TextDecoration => "inherit" },
                        span![C!["icon is-large"],
                            i![C!["fab fa-github fa-3x"]],
                        ]
                    ]
                ]
            ]
        ]
    ]
}