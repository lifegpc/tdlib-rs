use super::super::types::Server_DH_Inner_Data;
use crate::objects::traits::TypeId;
use bytes::BytesMut;
use openssl::{
    bn::{BigNum, BigNumContext},
    rand::rand_bytes,
};

/// Origin: `client_DH_inner_data#6643b654 nonce:int128 server_nonce:int128 retry_id:long g_b:string = Client_DH_Inner_Data;`
#[derive(Clone, Debug, tdlib_rs_impl::Deserialize, tdlib_rs_impl::Serialize)]
pub struct client_DH_inner_data {
    /// Value generated by client in [Step 1](https://core.telegram.org/mtproto/auth_key#dh-exchange-initiation).
    pub nonce: i128,
    /// Value received from server in [Step 2](https://core.telegram.org/mtproto/auth_key#dh-exchange-initiation).
    pub server_nonce: i128,
    /// Equal to zero at the time of the first attempt;
    /// otherwise, it is equal to auth_key_aux_hash from the previous failed attempt (see [Item 9](https://core.telegram.org/mtproto/auth_key#dh-key-exchange-complete)).
    pub retry_id: i64,
    /// `pow(g, b) mod dh_prime` (See [Step 6](https://core.telegram.org/mtproto/auth_key#presenting-proof-of-work-server-authentication))
    pub g_b: BytesMut,
    /// Used to calculate authroization key.
    #[skip_serialize]
    #[skip_deserialize]
    pub b: BytesMut,
}

impl client_DH_inner_data {
    /// Create a new instance.
    /// * `server_DH_inner_data` - Server's DH inner data received in [Step 5](https://core.telegram.org/mtproto/auth_key#presenting-proof-of-work-server-authentication)
    /// * `retry_id` - Equal to zero at the time of the first attempt;
    /// otherwise, it is equal to auth_key_aux_hash from the previous failed attempt (see [Item 9](https://core.telegram.org/mtproto/auth_key#dh-key-exchange-complete))
    pub fn new(
        server_inner_data: &Server_DH_Inner_Data,
        retry_id: i64,
    ) -> Result<Self, openssl::error::ErrorStack> {
        match server_inner_data {
            Server_DH_Inner_Data::Boxed(server_inner_data) => {
                let prime = BigNum::from_slice(&server_inner_data.dh_prime)?;
                let g = BigNum::from_u32(server_inner_data.g as u32)?;
                let mut b = BytesMut::with_capacity(256);
                b.resize(256, 0);
                rand_bytes(&mut b)?;
                let b = BigNum::from_slice(&b)?;
                let mut g_b = BigNum::new()?;
                let mut ctx = BigNumContext::new()?;
                g_b.mod_exp(&g, &b, &prime, &mut ctx)?;
                let g_b = g_b.to_vec();
                let g_b: &[u8] = &g_b;
                let g_b = BytesMut::from(g_b);
                let b = b.to_vec();
                let b: &[u8] = &b;
                let b = BytesMut::from(b);
                Ok(Self {
                    nonce: server_inner_data.nonce,
                    server_nonce: server_inner_data.server_nonce,
                    retry_id,
                    g_b,
                    b,
                })
            }
        }
    }
}

impl TypeId for client_DH_inner_data {
    fn type_id2() -> u32 {
        0x6643b654
    }
}
