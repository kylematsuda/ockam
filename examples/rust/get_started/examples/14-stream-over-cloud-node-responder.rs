use ockam::{route, stream::Stream, Context, Result, TcpTransport, TCP};
use ockam_get_started::Echoer;
use std::time::Duration;

#[ockam::node]
async fn main(ctx: Context) -> Result<()> {
    let tx_name = std::env::var("TX").ok();
    let rx_name = std::env::var("RX").ok();

    let tcp = TcpTransport::create(&ctx).await?;
    tcp.connect("127.0.0.1:4000").await?;

    // Start a printer
    ctx.start_worker("echoer", Echoer).await?;

    // Create the stream
    Stream::new(&ctx)?
        .with_interval(Duration::from_millis(100))
        .connect(
            route![(TCP, "127.0.0.1:4000")],
            // Stream name from THIS to OTHER
            tx_name,
            // Stream name from OTHER to THIS
            rx_name,
        )
        .await?;

    Ok(())
}
