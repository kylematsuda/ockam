/// This example uses the stream service to send messages between two
/// clients.  A stream is a buffered message sending channel, which
/// means that you can run `initiator` and `responder` in any order
/// you like.
use ockam::{route, stream::Stream, Context, Result, TcpTransport, TCP};
use std::time::Duration;

#[ockam::node]
async fn main(mut ctx: Context) -> Result<()> {
    let tx_name = std::env::var("TX").ok();
    let rx_name = std::env::var("RX").ok();

    let tcp = TcpTransport::create(&ctx).await?;
    tcp.connect("127.0.0.1:4000").await?;

    let (tx, _rx) = Stream::new(&ctx)?
        .with_interval(Duration::from_millis(100))
        .connect( // route, tx_name, rx_name
            route![(TCP, "127.0.0.1:4000")],
            // Stream name from THIS node to the OTHER node
            tx_name,
            // Stream name from OTHER to THIS
            rx_name,
        )
        .await?;

    ctx.send(tx.to_route().append("echoer"), "Hello World!".to_string())
        .await?;

    let reply = ctx.receive_block::<String>().await?;
    println!("Reply via stream: {}", reply);

    ctx.stop().await
}
