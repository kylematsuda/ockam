# Step 4 - Secure Channels

Secure channels are encrypted bi-directional message routes between two
entities. One entity acts as a listener in the secure channel protocol, and the
other is the initiator.

When a secure channel listener receives a request to start a new channel, it
checks a trust policy to ensure the initiating entity is allowed to connect. In
particular, an `IdentifierTrustPolicy` is used, which specifies a set of
permitted profile identifiers.

The secure channel creation protocol also verifies that the entity presenting a
profile identifier actually has possession of the keypair for that identifier.

The entities then run the key agreement protocol, which allows both entities to
securely agree upon a secret key without exchanging it over the network. The
algorithm used for key agreement is pluggable. By default, an implementation of
the Noise XX protocol is used. The Signal X3DH protocol is also available as a
crate add-on.

After the key agreement is done, the initiating entity starts a worker to
manage the secure channel. The address of this worker is used to send messages
through the channel. This address is included in routes just like transport and
other worker addresses.


## Creating a Secure Channel

Entities create a secure channel by calling `create_secure_channel_listener` on
the listening peer, and `Entity::create_secure_channel` on the initiating peer.

Creating the listener requires two parameters:
- The address of the secure channel being established
- A trust policy control on secure channel creation

Creating the initiator also requires two parameters:
- The route to the secure channel
- A trust policy control on secure channel creation


## Example: Echoer through Secure Channel

In this example, Alice creates a secure channel with Bob. Alice and Bob are on
different nodes, and use the TCP transport to route messages.


### Example: Bob (Listener)

```rust
use fresh::Echoer;  
use ockam::{  
    Context, Entity, NoOpTrustPolicy, Result, SecureChannel, SecureChannels, TcpTransport, Vault,  
};  
  
#[ockam::node]  
async fn main(mut ctx: Context) -> Result<()> {  
    // Create an echoer worker  
 ctx.start_worker("echoer", Echoer).await?;  
  
 let vault = Vault::create(&ctx)?;  
 let mut bob = Entity::create(&ctx, &vault)?;  
  
 bob.create_secure_channel_listener("secure_channel_listener", NoOpTrustPolicy)?;  
  
 // Initialize the TCP Transport.  
 let tcp = TcpTransport::create(&ctx).await?;  
  
 // Create a TCP listener and wait for incoming connections.  
 tcp.listen("127.0.0.1:4000").await?;  
  
 // Don't call ctx.stop() here so this node runs forever.  
 Ok(())  
}

```

### Example: Alice (Initiator)

```rust
use ockam::{  
    route, Context, Entity, NoOpTrustPolicy, Result, Route, SecureChannels, TcpTransport, Vault,  
 TCP,  
};  
  
#[ockam::node]  
async fn main(mut ctx: Context) -> Result<()> {  
    // Initialize the TCP Transport.  
 let tcp = TcpTransport::create(&ctx).await?;  
  
 // Create a TCP connection.  
 tcp.connect("127.0.0.1:4000").await?;  
  
 let vault = Vault::create(&ctx)?;  
 let mut alice = Entity::create(&ctx, &vault)?;  
  
 // Connect to a secure channel listener and perform a handshake.  
 let channel = alice.create_secure_channel(  
        // route to the secure channel listener  
 Route::new()  
            .append_t(TCP, "127.0.0.1:4000") // responder node  
 .append("secure_channel_listener"), // secure_channel_listener on responder node,  
 NoOpTrustPolicy,  
 )?;  
  
 // Send a message to the echoer worker via the channel.  
 ctx.send(route![channel, "echoer"], "Hello Ockam!".to_string())  
        .await?;  
  
 // Wait to receive a reply and print it.  
 let reply = ctx.receive::<String>().await?;  
 println!("App Received: {}", reply); // should print "Hello Ockam!"  
  
 // Stop all workers, stop the node, cleanup and return. ctx.stop().await  
}
```
