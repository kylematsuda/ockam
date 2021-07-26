```yaml
title: Entities
```

## Entities

The primary worker which uses the vault worker on behalf of a user is called an
Entity worker. Entities offer a small, simplified interface to more complex
security protocols. They provide the features of the underlying protocols,
while handling implementation details. The interaction between multiple parties
establishing trust is modeled using Entities.

Entities provide:

- Cryptographic key creation, rotation and retrieval
- Cryptographic proof creation and verification mechanism
- Secure Channel establishment
- Credential issuance and verification
- Change verification
- Contact management

Like most things in Ockam, an Entity is a worker. An entity worker is created
by calling the `Entity::create` function with the address of a Vault. A vault
provides secure storage for secrets. In this example, we create a software
vault worker for the entity.

```rust
use ockam::Entity;
...
let vault = Vault::create(&ctx)?;
let light_switch = Entity::create(&ctx, &vault)?;
```
