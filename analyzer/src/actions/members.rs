use common::Member;
use ipfs_api::IpfsClient;
use std::io::Cursor;
use crate::util;

pub async fn update_members(members: Vec<Member>, ipfs: &IpfsClient) -> String {
    let data = serde_json::to_string(&members).unwrap();
    let res = ipfs.add(Cursor::new(data)).await.unwrap();
    res.hash
}

pub async fn reconcile_members(ipfs: &IpfsClient, old_members_cid: String, new_members: Vec<Member>) -> Vec<Member> {
    let old_members = String::from_utf8(util::read_complete(ipfs.cat(&old_members_cid)).await).unwrap();
    let _ = ipfs.pin_rm(&format!("/ipfs/{}", old_members_cid), true).await.unwrap();
    let mut old_members = serde_json::from_str::<Vec<Member>>(&old_members).unwrap();

    if old_members == new_members {
        return new_members;
    }

    for member in new_members.into_iter() {
        if !old_members.contains(&member) {
            old_members.push(member)
        }
    }

    old_members
}