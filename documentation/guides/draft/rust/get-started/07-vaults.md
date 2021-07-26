```yaml
title: Vaults
```

## Vaults

Various Ockam protocols, like secure channels, key lifecycle, credential
exchange, device enrollment etc. depend on a variety of standard
cryptographic primitives or building blocks. Depending on the environment,
these building blocks may be provided by a software implementation or a
cryptographically capable hardware component.

To support a variety of security hardware, there is loose coupling between
Ockam security protocols' building blocks and the underlying specific hardware
implementation. This is achieved using an abstract notion called Vault. A
software vault worker implementation is available to Ockam nodes. Over time,
and with help from the Ockam open source community, we plan to add vaults for
several TEEs, TPMs, HSMs, and Secure Enclaves.

Creating a vault worker is done by calling `Vault::create`:

```rust
// Returns the address of a newly started vault worker
let vault = Vault::create(&ctx)?;
```

Vault workers are used by other workers to access secrets and perform
cryptographic operations. In a typical program, a vault worker is created and
provided to an entity. An entity and its profiles call the vault when needed.
