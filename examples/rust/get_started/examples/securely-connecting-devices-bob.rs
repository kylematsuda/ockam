use ockam::{
    route, Context, Entity, NoOpTrustPolicy, RemoteForwarder, Result, SecureChannels, TcpTransport,
    Vault, TCP,
};

#[ockam::node]
async fn main(mut ctx: Context) -> Result<()> {
    // Using a shared Hub Node.
    // You can create a personal node by going to https://hub.ockam.network
    let cloud_node_tcp_address = "54.151.52.111:4000";

    // Initialize the TCP Transport.
    let tcp = TcpTransport::create(&ctx).await?;

    let vault = Vault::create(&ctx).expect("failed to create vault");
    let mut bob = Entity::create(&ctx, &vault)?;

    // Create a secure channel listener at address "bob_secure_channel_listener"
    bob.create_secure_channel_listener("bob_secure_channel_listener", NoOpTrustPolicy)?;

    let forwarder = RemoteForwarder::create(
        &ctx,
        route![(TCP, cloud_node_tcp_address)],
        "bob_secure_channel_listener",
    )
    .await?;

    println!("Forwarding address: {}", forwarder.remote_address());

    let message = ctx.receive::<String>().await?;
    println!("Bob Received: {} from Alice via secure channel", message); // should print "Hello Ockam!"

    Ok(())
}
