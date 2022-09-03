mod error;

use bytes::BytesMut;
pub use error::EncryptError;
use openssl::{
    aes::{aes_ige, AesKey},
    bn::BigNum,
    pkey::Public,
    rsa::{Padding, Rsa},
    sha::{sha256, Sha256},
};

/// Used to generate key_aes_encrypted
fn rsa_pad_internal(
    data_with_padding: &BytesMut,
    data_pad_reversed: &BytesMut,
    server_public_key: &Rsa<Public>,
) -> Result<Option<BytesMut>, EncryptError> {
    let mut temp_key = BytesMut::with_capacity(32);
    temp_key.resize(32, 0);
    openssl::rand::rand_bytes(&mut temp_key)?;
    let temp_key2 = AesKey::new_encrypt(&temp_key)?;
    let mut data_with_hash = data_pad_reversed.clone();
    let mut hasher = Sha256::new();
    hasher.update(&temp_key);
    hasher.update(&data_with_padding);
    data_with_hash.extend_from_slice(&hasher.finish());
    let mut iv = BytesMut::with_capacity(32);
    iv.resize(32, 0);
    let mut aes_encrypted = BytesMut::with_capacity(224);
    aes_encrypted.resize(224, 0);
    aes_ige(
        &data_with_hash,
        &mut aes_encrypted,
        &temp_key2,
        &mut iv,
        openssl::symm::Mode::Encrypt,
    );
    let aes_encrypted_sha256 = sha256(&aes_encrypted);
    let mut temp_key_xor = temp_key;
    for i in 0..32 {
        temp_key_xor[i] ^= aes_encrypted_sha256[i];
    }
    let mut key_aes_encrypted = BytesMut::with_capacity(256);
    key_aes_encrypted.extend_from_slice(&temp_key_xor);
    key_aes_encrypted.extend_from_slice(&aes_encrypted);
    let n = server_public_key.n();
    let ne = BigNum::from_slice(&key_aes_encrypted)?;
    if &ne >= n {
        Ok(None)
    } else {
        Ok(Some(key_aes_encrypted))
    }
}

/// RSA_PAD in [step 4](https://core.telegram.org/mtproto/auth_key#presenting-proof-of-work-server-authentication).
/// * `data` - data to encrypt, shoule less or equal to 144 bytes
/// * `server_public_key` - server public key
pub fn rsa_pad(data: &[u8], server_public_key: &Rsa<Public>) -> Result<BytesMut, EncryptError> {
    if data.len() > 144 {
        return Err(EncryptError::DataTooLong);
    }
    let mut data_with_padding = BytesMut::with_capacity(192);
    data_with_padding.extend_from_slice(data);
    if data_with_padding.len() < 192 {
        data_with_padding.resize(192, 0);
        openssl::rand::rand_bytes(&mut data_with_padding[data.len()..])?;
    }
    let mut data_pad_reversed = data_with_padding.clone();
    data_pad_reversed.reverse();
    let key_aes_encrypted = loop {
        if let Some(key_aes_encrypted) =
            rsa_pad_internal(&data_with_padding, &data_pad_reversed, server_public_key)?
        {
            break key_aes_encrypted;
        }
    };
    let mut encrypted_data = BytesMut::with_capacity(256);
    encrypted_data.resize(256, 0);
    server_public_key.public_encrypt(&key_aes_encrypted, &mut encrypted_data, Padding::NONE)?;
    Ok(encrypted_data)
}
