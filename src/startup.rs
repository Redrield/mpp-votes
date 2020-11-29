use common::{Redirects, Member, Division};
use crate::util;
use flate2::read::GzDecoder;
use std::io::Read;
use futures_util::stream::Stream;
use futures_util::stream::StreamExt;

pub fn fetch_members(body: Vec<u8>) -> Vec<Member> {
    let json = String::from_utf8(body).unwrap();

    serde_json::from_str::<Vec<Member>>(&json).unwrap()
}

pub fn fetch_divisions(body: Vec<u8>) -> Vec<Division> {
    let mut decoder = GzDecoder::new(&body[..]);
    let mut json = String::new();
    decoder.read_to_string(&mut json);

    serde_json::from_str::<Vec<Division>>(&json).unwrap()
}