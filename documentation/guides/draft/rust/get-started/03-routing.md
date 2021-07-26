```yaml
title: Routing
```
# Routing

The path that a message takes through an Ockam network is called a route. A
message carries route meta-data that nodes use to determine the next hop toward
the destination. A route is a list of worker addresses. The order of addresses
in a route defines the path the message will take from its source to its
destination.

A message has two routes: the **onward route** and the **return route**. The
onward route specifies the path the message takes to the destination. When
a node receives a message to route, the head of the address list is removed
from the route. This address is used to determine the next destination route,
or destination worker.

The return route of a message represents the path back to the source worker,
and may differ from the onward route. When a message is routed through a node,
the node adds its own address to the return route. This ensures that there is a
valid, known return path for message replies. All messages sent in an Ockam
Network have a route. Many messages between local workers have short routes,
only indicating the address of another local Worker.

Routes are created with the `route` macro, which takes a list of addresses:

```rust
let route = route!["node_a", "node_b", "some_worker"];
```
