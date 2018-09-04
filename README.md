# channels

**experimental:** derive keys from keys. built on [redschnorr](https://github.com/lukeburns/redschnorr) for [hypercore-red](https://github.com/lukeburns/hypercore-red/tree/experiment).

[hypercore](https://github.com/mafintosh/hypercore) is a secure, distributed append-only log that can be replicated on p2p networks.

this library is a proof of concept meant to demonstrate the viability of deterministically generating namespaced subfeeds associated with a given feed, which is possible because hypercore feeds are identified by public keys.

this library works with an experimental and incompatible [fork](https://github.com/lukeburns/hypercore-red) of an implementation of [hypercore in rust](https://github.com/datrs/hypercore) that is still under development. it is [insecure](https://github.com/lukeburns/redschnorr), so don't use it.

## examples

#### public channels

```rust
// generate Alice's keypair { public, secret }
let alice = generate_keypair();
let channel = b"cats";

// derive the keypair for Alice's `cats` channel from Alice's secret key
let cats_secret_key = derive_channel_secret::<Sha512>(&alice.secret, channel).unwrap();
let cats_public_key = derive_public_key(&cats_secret_key).unwrap();

// derive the public key for Alice's `cats` channel from Alice's public key
let pub_cats_public_key = derive_channel_public::<Sha512>(&alice.public, channel).unwrap();

println!("{:?}", pub_cats_public_key == cats_public_key);
```

#### shared secret channels

```rust
// generate Alice and Bob's keypairs { public, secret } and derive their shared secret
let alice = generate_keypair();
let bob = generate_keypair();
let shared_secret = derive_shared_secret(&alice.secret, &bob.public).unwrap().to_bytes();

// derive Alice's private channels to and from Bob
let to_bob_secret = derive_channel_secret::<Sha512>(&alice.secret, &shared_secret).unwrap();
let to_bob_public = derive_public_key(&to_bob_secret).unwrap();
let from_bob_public = derive_channel_public::<Sha512>(&bob.public, &shared_secret).unwrap();

// derive Bob's private channels to and from Alice
let to_alice_secret = derive_channel_secret::<Sha512>(&bob.secret, &shared_secret).unwrap();
let to_alice_public = derive_public_key(&to_alice_secret).unwrap();
let from_alice_public = derive_channel_public::<Sha512>(&alice.public, &shared_secret).unwrap();

println!("{:?}", from_bob_public == to_alice_public);
println!("{:?}", from_alice_public == to_bob_public);
```
