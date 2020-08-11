use std::time::Duration;
use waves_protobuf_schemas::waves::events::grpc::blockchain_updates_api_client::BlockchainUpdatesApiClient;
use waves_protobuf_schemas::waves::events::grpc::SubscribeRequest;
use std::env::var;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = var("URL").unwrap_or("http://localhost:6881".to_owned());
    let start_height = var("START_HEIGHT").unwrap_or("1".to_owned()).parse::<i32>()?;
    let throttle_millis = var("THROTTLE_MILLIS").unwrap_or("0".to_owned()).parse::<u64>()?;

    let mut client = BlockchainUpdatesApiClient::connect(url).await?;

    let request = tonic::Request::new(SubscribeRequest {
        from_height: start_height,
        to_height: 0
    });

    let mut stream = client.subscribe(request).await?.into_inner();

    while let Some(msg) = stream.message().await? {
        let upd = msg.update.unwrap();
        println!("{:?}", &upd.height);

        if throttle_millis > 0 {
            tokio::time::delay_for(Duration::from_millis(throttle_millis)).await;
        }
    }

    Ok(())
}
