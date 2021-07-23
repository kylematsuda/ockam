use ockam::{
    Context, Entity, NoOpTrustPolicy, RemoteForwarder, Result, SecureChannels, TcpTransport, Vault,
};
use ockam_get_started::Echoer;

#[ockam::node]
async fn main(ctx: Context) -> Result<()> {
    // Using a shared Hub Node.
    // You can create a personal node by going to https://hub.ockam.network
    let cloud_node_tcp_address = "54.151.52.111:4000";

    // Initialize the TCP Transport.
    let tcp = TcpTransport::create(&ctx).await?;

    // Create a TCP connection to your cloud node.
    tcp.connect(cloud_node_tcp_address).await?;

    // Create an echoer worker
    ctx.start_worker("echoer", Echoer).await?;
    let vault = Vault::create(&ctx).expect("failed to create vault");
    let mut bob = Entity::create(&ctx, &vault)?;

    // Create a secure channel listener at address "bob_secure_channel_listener"
    bob.create_secure_channel_listener("bob_secure_channel_listener", NoOpTrustPolicy)?;

    let forwarder =
        RemoteForwarder::create(&ctx, route![(TCP, cloud_node_tcp_address)], "bob_secure_channel_listener")
            .await?;

    println!("Forwarding address: {}", forwarder.remote_address());

    Ok(())
}
