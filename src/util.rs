use seed::futures::{Stream, StreamExt};

pub async fn read_complete(stream: impl Stream<Item=Vec<u8>>) -> Vec<u8> {
    stream.collect::<Vec<Vec<u8>>>().await.into_iter().flatten().collect()
}