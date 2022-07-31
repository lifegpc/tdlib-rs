use crate::objects::traits::TypeId;

/// The first step to [DH exchange initiation](https://core.telegram.org/mtproto/auth_key#dh-exchange-initiation)
pub struct req_pq_multi {
    /// Selected randomly by the client (random number) and identifies the client within this communication
    pub nonce: i128,
}

impl TypeId for req_pq_multi {
    fn type_id(&self) -> u32 {
        0xbe7e8ef1
    }
}
