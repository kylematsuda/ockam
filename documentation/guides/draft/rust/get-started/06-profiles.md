```yaml
title: Profiles
```

## Profiles

A Profile is a specific identifier backed by a keypair. An Entity can have
multiple Profiles, by having multiple keypairs in the Vault.

The ability for an Entity to have multiple Profiles enhances the privacy of
an Entity. Two Profiles belonging to an Entity cannot be associated with one
another, or back to the Entity. This allows a single real user to use multiple
Profiles, each for a different identity scenario.

For example, a user may have a Manufacturer Identity for technical support, and
an Advertiser Identity for third party integrations.

Entities and Profiles implement the same APIs. In many Ockam APIs, Entities and
Profiles can be used interchangeably.

An Entity has a default Profile which is created automatically. The
`current_profile` function returns this profile. Profiles are created by
calling `create_profile`, and removed with `remove_profile`.

```rust
// Create an Entity
let mut alice = Entity::create(&ctx)?;

// Get the default profile
let alice_default_profile = alice.current_profile().unwrap();

// Create a new profile
let alice_manufacturer = alice.create_profile()?;

// Delete a profile
alice.remove_profile(alice_manufacturer.identifier()?)?;
```
