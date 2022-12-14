use super::super::types::{P_Q_inner_data, Server_DH_Inner_Data};
use crate::objects::traits::{Deserialize, Serialize, TypeId};
use bytes::BytesMut;

/// Decrypt Error
#[derive(Debug, derive_more::From)]
pub enum DecryptError {
    /// OpenSSL Error
    OpenSSLError(openssl::error::ErrorStack),
    /// Aes Key error
    AesKeyError(openssl::aes::KeyError),
    /// `nonce` or `server_nonce` mismatched
    Mismatched,
    /// Deserialize error
    DeserializeError(crate::objects::DeserializeError),
}

/// Origin: `server_DH_params_ok#d0e8075c nonce:int128 server_nonce:int128 encrypted_answer:string = Server_DH_Params;`
#[derive(Clone, Debug, tdlib_rs_impl::Deserialize, tdlib_rs_impl::Serialize)]
pub struct server_DH_params_ok {
    /// Value generated by client in [Step 1](https://core.telegram.org/mtproto/auth_key#dh-exchange-initiation).
    pub nonce: i128,
    /// Value received from server in [Step 2](https://core.telegram.org/mtproto/auth_key#dh-exchange-initiation).
    pub server_nonce: i128,
    /// Encrypted server answer.
    pub encrypted_answer: BytesMut,
}

impl server_DH_params_ok {
    /// Decrypt the answer.
    /// * `p_q_inner_data`: Data sended in [Step 4](https://core.telegram.org/mtproto/auth_key#presenting-proof-of-work-server-authentication).
    pub fn decrypt_answer(
        &self,
        p_q_inner_data: &P_Q_inner_data,
    ) -> Result<Server_DH_Inner_Data, DecryptError> {
        let server_nonce = match p_q_inner_data {
            P_Q_inner_data::P_Q_inner_data_dc(v) => v.server_nonce,
            P_Q_inner_data::P_Q_inner_data_temp_dc(v) => v.server_nonce,
        };
        if server_nonce != self.server_nonce {
            return Err(DecryptError::Mismatched);
        }
        let nonce = match p_q_inner_data {
            P_Q_inner_data::P_Q_inner_data_dc(v) => v.nonce,
            P_Q_inner_data::P_Q_inner_data_temp_dc(v) => v.nonce,
        };
        if nonce != self.nonce {
            return Err(DecryptError::Mismatched);
        }
        let new_nonce = match p_q_inner_data {
            P_Q_inner_data::P_Q_inner_data_dc(v) => &v.new_nonce,
            P_Q_inner_data::P_Q_inner_data_temp_dc(v) => &v.new_nonce,
        };
        let new_nonce = new_nonce.serialize_to_bytes();
        let server_nonce = self.server_nonce.serialize_to_bytes();
        let mut hasher = openssl::sha::Sha1::new();
        hasher.update(&new_nonce);
        hasher.update(&server_nonce);
        let mut tmp_aes_key = BytesMut::with_capacity(32);
        tmp_aes_key.extend_from_slice(&hasher.finish());
        let mut hasher = openssl::sha::Sha1::new();
        hasher.update(&server_nonce);
        hasher.update(&new_nonce);
        let hashre = hasher.finish();
        tmp_aes_key.extend_from_slice(&hashre[0..12]);
        let mut tmp_aes_iv = BytesMut::with_capacity(32);
        tmp_aes_iv.extend_from_slice(&hashre[12..20]);
        let mut hasher = openssl::sha::Sha1::new();
        hasher.update(&new_nonce);
        hasher.update(&new_nonce);
        tmp_aes_iv.extend_from_slice(&hasher.finish());
        tmp_aes_iv.extend_from_slice(&new_nonce[0..4]);
        let mut answer_with_hash = BytesMut::with_capacity(self.encrypted_answer.len());
        answer_with_hash.resize(self.encrypted_answer.len(), 0);
        let aes_key = openssl::aes::AesKey::new_decrypt(&tmp_aes_key)?;
        openssl::aes::aes_ige(
            &self.encrypted_answer,
            &mut answer_with_hash,
            &aes_key,
            &mut tmp_aes_iv,
            openssl::symm::Mode::Decrypt,
        );
        let inner = Server_DH_Inner_Data::deserialize_from_bytes(&answer_with_hash[20..])?;
        let answer = inner.serialize_to_bytes();
        let answer_hash = openssl::sha::sha1(&answer);
        if answer_hash != answer_with_hash[0..20] {
            return Err(DecryptError::Mismatched);
        }
        Ok(inner)
    }
}

impl TypeId for server_DH_params_ok {
    fn type_id2() -> u32 {
        0xd0e8075c
    }
}
