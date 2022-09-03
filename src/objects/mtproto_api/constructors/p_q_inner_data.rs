use super::resPQ;
use crate::objects::{base::I256, traits::TypeId};
use bytes::BytesMut;
use rand::{Rng, SeedableRng};
use std::convert::TryFrom;

#[derive(Clone, Debug, tdlib_rs_impl::Deserialize, tdlib_rs_impl::Serialize)]
/// Used in [Server Authentication](https://core.telegram.org/mtproto/auth_key#presenting-proof-of-work-server-authentication) to create a permanent authorization key.
///
/// Origin: `p_q_inner_data_dc#a9f55f95 pq:string p:string q:string nonce:int128 server_nonce:int128 new_nonce:int256 dc:int = P_Q_inner_data;`
pub struct p_q_inner_data_dc {
    /// pq in [Step 2](https://core.telegram.org/mtproto/auth_key#dh-exchange-initiation).
    /// (See also: [super::resPQ])
    pub pq: BytesMut,
    /// First prime cofactor.
    pub p: BytesMut,
    /// Second prime cofactor.
    pub q: BytesMut,
    /// Value generated by client in [Step 1](https://core.telegram.org/mtproto/auth_key#dh-exchange-initiation).
    /// (See also: [resPQ])
    pub nonce: i128,
    /// Value received from server in [Step 2](https://core.telegram.org/mtproto/auth_key#dh-exchange-initiation).
    /// (See also: [resPQ])
    pub server_nonce: i128,
    /// Client-generated random number
    pub new_nonce: I256,
    pub dc: i32,
}

impl TryFrom<&resPQ> for p_q_inner_data_dc {
    type Error = super::FactorizeError;
    fn try_from(value: &resPQ) -> Result<Self, Self::Error> {
        let (p, q) = value.pq_factorize()?;
        let p: &[u8] = p.as_ref();
        let q: &[u8] = q.as_ref();
        Ok(Self {
            pq: value.pq.clone(),
            p: BytesMut::from(p),
            q: BytesMut::from(q),
            nonce: value.nonce,
            server_nonce: value.server_nonce,
            new_nonce: rand::rngs::StdRng::from_entropy().gen(),
            dc: -1,
        })
    }
}

impl TypeId for p_q_inner_data_dc {
    fn type_id2() -> u32 {
        0xa9f55f95
    }
}

#[derive(Clone, Debug, tdlib_rs_impl::Deserialize, tdlib_rs_impl::Serialize)]
/// Used in [Server Authentication](https://core.telegram.org/mtproto/auth_key#presenting-proof-of-work-server-authentication) to create a temporary authorization key,
/// that are only stored in the server RAM and are discarded after at most `expires_in` seconds.
///
/// Origin: `p_q_inner_data_temp_dc#56fddf88 pq:string p:string q:string nonce:int128 server_nonce:int128 new_nonce:int256 dc:int expires_in:int = P_Q_inner_data;`
pub struct p_q_inner_data_temp_dc {
    /// pq in [Step 2](https://core.telegram.org/mtproto/auth_key#dh-exchange-initiation).
    /// (See also: [super::resPQ])
    pub pq: BytesMut,
    /// First prime cofactor.
    pub p: BytesMut,
    /// Second prime cofactor.
    pub q: BytesMut,
    /// Value generated by client in [Step 1](https://core.telegram.org/mtproto/auth_key#dh-exchange-initiation).
    /// (See also: [super::resPQ])
    pub nonce: i128,
    /// Value received from server in [Step 2](https://core.telegram.org/mtproto/auth_key#dh-exchange-initiation).
    /// (See also: [super::resPQ])
    pub server_nonce: i128,
    /// Client-generated random number
    pub new_nonce: I256,
    pub dc: i32,
    /// Discarded after at most `expires_in` seconds.
    pub expires_in: i32,
}

impl TryFrom<&resPQ> for p_q_inner_data_temp_dc {
    type Error = super::FactorizeError;
    fn try_from(value: &resPQ) -> Result<Self, Self::Error> {
        let (p, q) = value.pq_factorize()?;
        let p: &[u8] = p.as_ref();
        let q: &[u8] = q.as_ref();
        Ok(Self {
            pq: value.pq.clone(),
            p: BytesMut::from(p),
            q: BytesMut::from(q),
            nonce: value.nonce,
            server_nonce: value.server_nonce,
            new_nonce: rand::rngs::StdRng::from_entropy().gen(),
            dc: -1,
            expires_in: 120,
        })
    }
}

impl TypeId for p_q_inner_data_temp_dc {
    fn type_id2() -> u32 {
        0x56fddf88
    }
}
