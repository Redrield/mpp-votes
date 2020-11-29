use tokio::stream::Stream;
use futures_util::stream::StreamExt;
use bytes::Bytes;
use ipfs_api::response::Error;

pub async fn read_complete(stream: impl Stream<Item=Result<Bytes, Error>>) -> Vec<u8> {
    stream.map(|b| b.unwrap().to_vec()).collect::<Vec<Vec<u8>>>().await.into_iter().flatten().collect::<Vec<u8>>()
}