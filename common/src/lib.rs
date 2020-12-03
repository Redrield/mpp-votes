use serde::{Serialize, Deserialize};
use std::fmt;

pub mod search;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Party {
    OPC,
    NDP,
    LIB,
    GRN,
    Independent,
    Unknown,
}

impl fmt::Display for Party {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Party::OPC => f.write_str("Progressive Conservative Party"),
            Party::NDP => f.write_str("New Democratic Party"),
            Party::LIB => f.write_str("Liberal Party"),
            Party::GRN => f.write_str("Green Party"),
            Party::Independent => f.write_str("Independent MPP"),
            Party::Unknown => f.write_str("Unknown Party"),
        }
    }
}

impl From<&str> for Party {
    fn from(s: &str) -> Self {
        match s {
            "Progressive Conservative Party of Ontario" => Party::OPC,
            "New Democratic Party of Ontario" => Party::NDP,
            "Ontario Liberal Party" => Party::LIB,
            "Green Party of Ontario" => Party::GRN,
            "Independent" => Party::Independent,
            _ => Party::Unknown,
        }
    }
}

impl Party {
    pub fn as_str(&self) -> &str {
        match self {
            Party::OPC => "Progressive Conservative Party",
            Party::NDP => "New Democratic Party",
            Party::LIB => "Liberal Party",
            Party::GRN => "Green Party",
            Party::Independent => "Independent",
            Party::Unknown => "Unknown"
        }
    }

    pub fn favourite_colour(&self) -> &str {
        match self {
            Party::OPC => "has-background-opc",
            Party::NDP => "has-background-ndp",
            Party::LIB => "has-background-lib",
            Party::GRN => "has-background-grn",
            Party::Independent => "has-background-ind",
            Party::Unknown => "black",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Member {
    pub full_name: String,
    pub party: Party,
    pub riding: String,
}

impl Member {
    pub fn last_name(&self) -> String {
        self.full_name
            .split_whitespace()
            .last()
            .unwrap()
            .to_string()
    }
}


#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Division {
    pub date: String,
    pub topic: String,
    pub ayes: Vec<Member>,
    pub nays: Vec<Member>,
}
