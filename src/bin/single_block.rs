use std::time::Instant;
use waves_protobuf_schemas::waves::events::grpc::blockchain_updates_api_client::BlockchainUpdatesApiClient;
use waves_protobuf_schemas::waves::events::grpc::{GetBlockUpdateRequest, GetBlockUpdateResponse};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut client = BlockchainUpdatesApiClient::connect("https://blockchain-updates-stagenet.waves.exchange").await?;
    let mut client = BlockchainUpdatesApiClient::connect("http://localhost:6881").await?;

    let request = tonic::Request::new(GetBlockUpdateRequest {
        height: 150000
    });

    let response = client.get_block_update(request).await?;

    println!("{:?}", response);
    
    Ok(())
}
