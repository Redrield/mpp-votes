use seed::{*, prelude::*};
use crate::Msg;

pub fn content() -> Node<Msg> {
    div![C!["container"],
        section![C!["hero"],
            div![C!["hero-body"],
                h1![C!["title has-text-centered"], fl!("faqs-title")]
            ]
        ],
        section![C!["section"],
            div![C!["content"],
                ul![
                    li![
                        div![C!["content"],
                            p![fl!("faqs-faq-info-title")],
                            p![fl!("faqs-faq-info-body")],
                        ]
                    ],
                    br![],
                    li![
                        div![C!["content"],
                            p![fl!("faqs-faq-all-votes-title")],
                            p![fl!("faqs-faq-all-votes-body1"), " ", a![attrs!{ At::Href => "https://github.com/Redrield/onvotes" }, fl!("faqs-faq-all-votes-body2")], " ", fl!("faqs-faq-all-votes-body3")]
                        ]
                    ],
                    br![],
                    li![
                        div![C!["content"],
                            p![fl!("faqs-faq-how-title")],
                            p![fl!("faqs-faq-how-body")]
                        ]
                    ]
                ]
            ]
        ]
    ]
}