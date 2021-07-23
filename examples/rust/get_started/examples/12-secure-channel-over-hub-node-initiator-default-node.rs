use ockam::{
    route, Address, Context, Entity, NoOpTrustPolicy, Result, SecureChannels, TcpTransport, Vault,
    TCP,
};

#[ockam::node]
async fn main(mut ctx: Context) -> Result<()> {
    // Using a shared Hub Node.
    // You can create a personal node by going to https://hub.ockam.network
    let cloud_node_tcp_address = "54.151.52.111:4000";

    let secure_channel_listener_forwarding_address =
        "<Paste the forwarding address of the secure channel here>";

    // Initialize the TCP Transport.
    let tcp = TcpTransport::create(&ctx).await?;

    // Create a TCP connection to your cloud node.
    tcp.connect(cloud_node_tcp_address).await?;

    let vault = Vault::create(&ctx).expect("failed to create vault");
    let mut alice = Entity::create(&ctx, &vault)?;
    let cloud_node_address: Address = (TCP, cloud_node_tcp_address).into();
    let cloud_node_route = route![
        cloud_node_address,
        secure_channel_listener_forwarding_address
    ];

    let channel = alice.create_secure_channel(cloud_node_route, NoOpTrustPolicy)?;

    let echoer_route = route![channel, "echoer"];

    ctx.send(echoer_route, "Hello world!".to_string()).await?;

    // Wait to receive a reply and print it.
    let reply = ctx.receive::<String>().await?;
    println!("App Received: {}", reply); // should print "Hello Ockam!"

    // Stop all workers, stop the node, cleanup and return.
    ctx.stop().await
}
