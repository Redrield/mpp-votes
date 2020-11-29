use common::Division;
use ipfs_api::IpfsClient;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::{Cursor, Write};

pub async fn update_votes(mut old: String, divisions: Vec<Division>, ipfs: &IpfsClient) -> String {
    let json = serde_json::to_string(&divisions).unwrap();
    if old.trim() == "[]" {
        log::info!("Publishing new data to IPFS...");
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(json.as_bytes()).unwrap();
        let data = encoder.finish().unwrap();
        let res = ipfs.add(Cursor::new(data)).await.unwrap();
        log::info!("{:?}", res);
        log::info!("Published at {}", res.hash);
        res.hash
    } else {
        let len = old.len();
        old.reserve(json.len() + 1);
        // Safety: All the bytes written in the lifetime of this cursor are valid UTF8
        let mut output = Cursor::new(unsafe { old.as_mut_vec() });
        // Overwrite the end of the array, add a comma, and the data collected now
        output.set_position((len - 2) as u64);
        output.write(&[b',']).unwrap();
        output.write_all(&json[1..].as_bytes()).unwrap();
        log::info!("Data appended to previous entry. Publishing to IPFS...");
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(old.as_bytes()).unwrap();
        let data = encoder.finish().unwrap();
        let res = ipfs.add(Cursor::new(data)).await.unwrap();
        log::info!("Data uploaded {}", res.hash);
        res.hash
    }
}