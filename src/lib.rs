#[macro_use] extern crate failure;

extern crate digest;
extern crate generic_array;
extern crate redschnorr;
extern crate curve25519_dalek;

use failure::Error;
use digest::Digest;
use generic_array::typenum::U64;

use curve25519_dalek::constants;
use curve25519_dalek::ristretto::CompressedRistretto;
use curve25519_dalek::scalar::Scalar;

use redschnorr::{SecretKey, PublicKey};

#[derive(Debug)] // we derive Default in order to use the clear() method in Drop
pub struct RelationshipKeys {
    pub secret: SecretKey,
    pub public: PublicKey
}

pub fn derive_channel_secret<D>(secret: &SecretKey, bytes: &[u8]) -> Result<SecretKey, Error>
        where D: Digest<OutputSize = U64> + Default {
    let sk = Scalar::from_bytes_mod_order(*secret.as_bytes());
    let s = Scalar::hash_from_bytes::<D>(bytes);
    let channel_secret = s*sk;

    match SecretKey::from_bytes(channel_secret.as_bytes()) {
        Ok(x) => return Ok(x),
        Err(e) => bail!(e)
    }
}

pub fn derive_channel_public<D>(public: &PublicKey, bytes: &[u8]) -> Result<PublicKey, Error>
        where D: Digest<OutputSize = U64> + Default {
    let pk = match CompressedRistretto(public.to_bytes()).decompress() {
        Some(x) => x,
        None    => bail!("Point decompression error"),
    };
    let s = Scalar::hash_from_bytes::<D>(bytes);
    let channel_public = s*pk;

    match PublicKey::from_bytes(channel_public.compress().as_bytes()) {
        Ok(x) => return Ok(x),
        Err(e) => bail!(e)
    }
}

pub fn derive_relationship_keys<D>(secret: &SecretKey, public: &PublicKey) -> Result<RelationshipKeys, Error>
        where D: Digest<OutputSize = U64> + Default {
    let sk = Scalar::from_bytes_mod_order(*secret.as_bytes());
    let pk = match CompressedRistretto(public.to_bytes()).decompress() {
        Some(x) => x,
        None    => bail!("Point decompression error"),
    };

    let bytes = match derive_shared_secret(secret, public) {
        Ok(x) => x.to_bytes(),
        Err(e) => bail!(e)
    };

    let outbox = match derive_channel_secret::<D>(secret, &bytes) {
        Ok(x) => x,
        Err(e) => bail!(e)
    };

    let inbox = match derive_channel_public::<D>(public, &bytes) {
        Ok(x) => x,
        Err(e) => bail!(e)
    };

    Ok(RelationshipKeys { secret: outbox, public: inbox })
}

pub fn derive_shared_secret (secret: &SecretKey, public: &PublicKey) -> Result<SecretKey, Error> {
    let sk = Scalar::from_bytes_mod_order(*secret.as_bytes());
    let pk = match CompressedRistretto(public.to_bytes()).decompress() {
        Some(x) => x,
        None    => bail!("Point decompression error"),
    };

    let shared_bytes = (sk * pk).compress().to_bytes();
    let shared_secret = match SecretKey::from_bytes(&shared_bytes) {
        Ok(x) => x,
        Err(e) => bail!(e),
    };

    Ok(shared_secret)
}

pub fn derive_public_key (secret: &SecretKey) -> Result<PublicKey, Error> {
    let sk = Scalar::from_bytes_mod_order(*secret.as_bytes());
    let pk = &sk * &constants::RISTRETTO_BASEPOINT_TABLE;

    match PublicKey::from_bytes(pk.compress().as_bytes()) {
        Ok(x) => return Ok(x),
        Err(e) => bail!(e)
    }
}
