use humantime::format_duration;
use std::env::var;
use std::time::Duration;
use waves_protobuf_schemas::waves::events::grpc::blockchain_updates_api_client::BlockchainUpdatesApiClient;
use waves_protobuf_schemas::waves::events::grpc::SubscribeRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = var("URL")
        .map(|s| {
            println!("URL {}", s);
            s
        })
        .unwrap_or("http://localhost:6881".to_owned());

    let from_height = var("FROM_HEIGHT")
        .map(|s| {
            println!("FROM_HEIGHT {}", s);
            s.parse::<i32>().expect("FROM_HEIGHT must be integer")
        })
        .unwrap_or(1);

    let to_height = var("TO_HEIGHT")
        .map(|s| {
            println!("TO_HEIGHT {}", s);
            s.parse::<i32>().expect("TO_HEIGHT must be integer")
        })
        .unwrap_or(0);

    let silent = var("SILENT")
        .map(|s| {
            println!("SILENT {}", s);
            s.parse::<bool>().expect("SILENT must be boolean")
        })
        .unwrap_or(false);

    let throttle_millis = var("THROTTLE_MILLIS")
        .map(|s| {
            println!("THROTTLE_MILLIS {}", s);
            s.parse::<u64>()
                .expect("THROTTLE_MILLIS must be unsigned long")
        })
        .unwrap_or(0);

    let mut client = BlockchainUpdatesApiClient::connect(url).await?;

    let request = SubscribeRequest {
        from_height,
        to_height,
    };

    let streaming_started_at = std::time::Instant::now();

    let mut stream = client.subscribe(request).await?.into_inner();

    while let Some(msg) = stream.message().await? {
        let upd = msg.update.unwrap();

        if !silent {
            println!("{:?}", &upd.height);
        }

        if throttle_millis > 0 {
            tokio::time::sleep(Duration::from_millis(throttle_millis)).await;
        }
    }

    println!(
        "Finished streaming in {}",
        format_duration(Duration::from_millis(
            streaming_started_at.elapsed().as_millis() as u64
        ))
    );

    Ok(())
}
