use std::env::var;
use waves_protobuf_schemas::waves::events::grpc::blockchain_updates_api_client::BlockchainUpdatesApiClient;
use waves_protobuf_schemas::waves::events::grpc::GetBlockUpdateRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = var("URL")
        .map(|s| {
            println!("URL {}", s);
            s
        })
        .unwrap_or("http://localhost:6881".to_owned());

    let height = var("HEIGHT")
        .map(|s| {
            println!("HEIGHT {}", s);
            s.parse::<i32>().expect("FROM_HEIGHT must be integer")
        })
        .unwrap_or(1);

    let mut client = BlockchainUpdatesApiClient::connect(url).await?;

    let request = GetBlockUpdateRequest { height };

    let response = client.get_block_update(request).await?;

    println!("{:?}", response);

    Ok(())
}
