use protos::protos::{kvrpcpb::GetRequest, tinykvpb::tiny_kv_client::TinyKvClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = TinyKvClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(GetRequest {
        context: None,
        key: vec![1, 2, 3],
        version: 1,
    });

    let response = client.kv_get(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
