use waves_protobuf_schemas::waves::events::grpc::blockchain_updates_api_client::BlockchainUpdatesApiClient;
use waves_protobuf_schemas::waves::events::grpc::GetBlockUpdateRequest;
use std::env::var;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = var("URL").unwrap_or("http://localhost:6881".to_owned());
    let mut client = BlockchainUpdatesApiClient::connect(url).await?;

    let request = tonic::Request::new(GetBlockUpdateRequest {
        height: 150000
    });

    let response = client.get_block_update(request).await?;

    println!("{:?}", response);
    
    Ok(())
}
