use seed::{*, prelude::*};
use crate::Msg;

pub fn content() -> Node<Msg> {
    div![C!["container"],
        section![C!["hero"],
            div![C!["hero-body"],
                h1![C!["title has-text-centered"], "Frequently Asked Questions"]
            ]
        ],
        section![C!["section"],
            div![C!["content"],
                ul![
                    li![
                        div![C!["content"],
                            p!["Where does this information come from?"],
                            p!["The votes are extracted from a document called Hansard published by the Legislative Assembly of Ontario.\
                            Hansard documents the proceedings of a day, and includes records of any votes in a given day, and how members voted."]
                        ]
                    ],
                    br![],
                    li![
                        div![C!["content"],
                            p!["Are all votes recorded here?"],
                            p!["I do my best to capture every vote that has happened in Parliament and index them here, but there are some times when something slips between the cracks.\
                            One example is very long motions documenting changes to standing rules. If there are any votes of consequence that should be documented here, but are missing, submit an issue at ", a![attrs!{ At::Href => "https://github.com/Redrield/onvotes" }, "the project page"], " so that I can improve the algorithm and capture the vote in question."]
                        ]
                    ],
                    br![],
                    li![
                        div![C!["content"],
                            p!["How does it work?"],
                            p!["The code that runs ONVotes is all available on GitHub. The repository contains details about how it's structured, and what parts of the project do what."]
                        ]
                    ]
                ]
            ]
        ]
    ]
}