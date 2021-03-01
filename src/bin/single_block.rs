use waves_protobuf_schemas::waves::events::grpc::blockchain_updates_api_client::BlockchainUpdatesApiClient;
use waves_protobuf_schemas::waves::events::grpc::GetBlockUpdateRequest;
use std::env::var;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = var("URL").unwrap_or("http://localhost:6881".to_owned());
    let height = var("HEIGHT").unwrap_or("1".to_owned()).parse::<i32>()?;
    let mut client = BlockchainUpdatesApiClient::connect(url).await?;

    let request = tonic::Request::new(GetBlockUpdateRequest {
        height: height
    });

    let response = client.get_block_update(request).await?;

    println!("{:?}", response);
    
    Ok(())
}
