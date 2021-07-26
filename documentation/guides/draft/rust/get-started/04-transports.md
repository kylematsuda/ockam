```yaml
title: Transports
```

# Transports

Ockam Transports are logical connections between Ockam Nodes. Ockam Transports
are an abstraction on top of physical transport protocols. The Ockam TCP
Transport is an implementation of an Ockam Transport using the TCP protocol.
This functionality is available in the `ockam_transport_tcp` crate, and is
included in the standard feature set of the top level `ockam` crate.

## Using the TCP Transport

The Ockam TCP Transport API fundamental type is `TcpTransport`. This type
provides the ability to create, connect, and listen for TCP connections. To
create a TCP transport, the Context is passed to the `create` function:

```rust
let tcp = TcpTransport::create(&ctx).await
```

The return value of `create` is a handle to the transport itself, which is used
for `connect` and `listen` calls. Listening on a local port is accomplished by
using the `listen` method. This method takes a string containing the IP address
and port, delimited by `:`. For example, this statement will listen on
localhost port 3000:

```rust
tcp.listen("127.0.0.1:3000").await
```

## Routing over Transports

Transports are implemented as workers, and have a unique address. The transport
address is used in routes to indicate that the message must be routed to the
remote peer.

Transport addresses also encode a unique protocol identifier. This identifier
is prefixed to the beginning of an address, followed by a `#`. The portion of
an address after the `#` is transport protocol specific. The TCP transport has
a transport protocol identifier of `1`, which is also aliased to the constant
`TCP`. The actual address uses the familiar `IP:PORT` format. A complete TCP
transport address could appear such as `1#127.0.0.1:3000`.

Transport addresses can be created using a tuple syntax to specify both
protocol id (TCP) and address:

```rust
// Implicit conversion from tuple to address
let route = route![(TCP, "10.0.0.1:8000")];
```

To send a message to a worker on another node connected by a transport, the
address of the transport is added to the route first, followed by the address
of the destination worker.

```rust
// This route forwards a message to the remote TCP peer Node
// and then to Worker "b"
let route = route![(TCP, "127.0.0.1:3000"), "b"]
```
