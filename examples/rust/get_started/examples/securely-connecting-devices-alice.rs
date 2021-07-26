use ockam::{
    route, Address, Context, Entity, NoOpTrustPolicy, Result, SecureChannels, TcpTransport, Vault,
    TCP,
};

#[ockam::node]
async fn main(mut ctx: Context) -> Result<()> {
    // Using a shared Hub Node.
    // You can create a personal node by going to https://hub.ockam.network
    let cloud_node_tcp_address = "54.151.52.111:4000";

    let forwarding_address = "<Paste the forwarding address of Bob here>";

    // Initialize the TCP Transport.
    let tcp = TcpTransport::create(&ctx).await?;

    let vault = Vault::create(&ctx).expect("failed to create vault");
    let mut alice = Entity::create(&ctx, &vault)?;

    let cloud_node_route = route![(TCP, cloud_node_tcp_address), forwarding_address];
    let channel = alice.create_secure_channel(cloud_node_route, NoOpTrustPolicy)?;

    ctx.send(route![channel, "app"], "Hello world!".to_string())
        .await?;

    Ok(())
}
