use crate::objects::{base::I256, traits::TypeId};
use bytes::BytesMut;

#[derive(Clone, Debug, tdlib_rs_impl::Deserialize, tdlib_rs_impl::Serialize)]
/// Used in [Server Authentication](https://core.telegram.org/mtproto/auth_key#presenting-proof-of-work-server-authentication).
///
/// Origin: `p_q_inner_data_dc#a9f55f95 pq:string p:string q:string nonce:int128 server_nonce:int128 new_nonce:int256 dc:int = P_Q_inner_data;`
pub struct p_q_inner_data_dc {
    pub pq: BytesMut,
    pub p: BytesMut,
    pub q: BytesMut,
    pub nonce: i128,
    pub server_nonce: i128,
    pub new_nonce: I256,
    pub dc: i32,
}

impl TypeId for p_q_inner_data_dc {
    fn type_id2() -> u32 {
        0xa9f55f95
    }
}

#[derive(Clone, Debug, tdlib_rs_impl::Deserialize, tdlib_rs_impl::Serialize)]
/// Origin: `p_q_inner_data_temp_dc#56fddf88 pq:string p:string q:string nonce:int128 server_nonce:int128 new_nonce:int256 dc:int expires_in:int = P_Q_inner_data;`
pub struct p_q_inner_data_temp_dc {
    pub pq: BytesMut,
    pub p: BytesMut,
    pub q: BytesMut,
    pub nonce: i128,
    pub server_nonce: i128,
    pub new_nonce: I256,
    pub dc: i32,
    pub expires_in: i32,
}

impl TypeId for p_q_inner_data_temp_dc {
    fn type_id2() -> u32 {
        0x56fddf88
    }
}
