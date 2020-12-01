use common::{Redirects, Member, Division};
use crate::util;
use flate2::read::GzDecoder;
use std::io::Read;
use futures_util::stream::Stream;
use futures_util::stream::StreamExt;
use fst::{Map, MapBuilder};
use std::borrow::Cow;


pub fn create_fst(divisions: &Vec<Division>) -> Map<Vec<u8>> {
    let mut mem = MapBuilder::memory();
    for (i, div) in divisions.iter().enumerate() {
        mem.insert(&div.topic.to_lowercase(), i as u64);
    }

    mem.into_map()
}