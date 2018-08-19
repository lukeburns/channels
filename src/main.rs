extern crate channels;

use channels::{
    derive_channel_public,
    derive_channel_secret,
    derive_shared_secret,
    derive_public_key
};

extern crate redschnorr;
extern crate rand;
extern crate sha2;

use rand::OsRng;
use sha2::Sha512;
use redschnorr::Keypair;

fn main () {
    // Generate Keypairs
    // -----------------

    let mut cspring1: OsRng = OsRng::new().unwrap();
    let mut cspring2: OsRng = OsRng::new().unwrap();
    let alice = Keypair::generate::<Sha512, _>(&mut cspring1);
    let bob = Keypair::generate::<Sha512, _>(&mut cspring2);

    // Public Channels
    // ---------------

    let channel = b"cats";

    // derive the keypair for Alice's `cats` channel from Alice's secret key
    let cats_secret_key = derive_channel_secret::<Sha512>(&alice.secret, channel).unwrap();
    let cats_public_key = derive_public_key(&cats_secret_key).unwrap();

    // derive the public key for Alice's `cats` channel from Alice's public key
    let pub_cats_public_key = derive_channel_public::<Sha512>(&alice.public, channel).unwrap();

    println!("{:?}", pub_cats_public_key == cats_public_key);

    // Secret Channels
    // ---------------

    // derive Alice and Bob's shared secret
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
}
