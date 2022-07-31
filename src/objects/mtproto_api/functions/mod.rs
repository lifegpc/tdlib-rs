use crate::objects::traits::TypeId;
use rand::{Rng, SeedableRng};

#[derive(Clone, Debug, tdlib_rs_impl::Serialize)]
/// The first step to [DH exchange initiation](https://core.telegram.org/mtproto/auth_key#dh-exchange-initiation)
///
/// Origin: `req_pq_multi#be7e8ef1 nonce:int128 = ResPQ;`
pub struct req_pq_multi {
    /// Selected randomly by the client (random number) and identifies the client within this communication
    pub nonce: i128,
}

impl req_pq_multi {
    /// Create a new instance.
    pub fn new() -> Self {
        Self {
            nonce: rand::rngs::StdRng::from_entropy().gen(),
        }
    }
}

impl TypeId for req_pq_multi {
    fn type_id(&self) -> u32 {
        0xbe7e8ef1
    }
}
