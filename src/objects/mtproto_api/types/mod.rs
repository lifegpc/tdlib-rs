/// RSA public key
mod rsa_public_key;

use super::constructors::*;
use crate::objects::{
    traits::{Deserialize, TypeId},
    DeserializeError,
};
pub use rsa_public_key::RSAPublicKey;

#[derive(
    Clone, Debug, tdlib_rs_impl::OptDeserialize, tdlib_rs_impl::From1, tdlib_rs_impl::Serialize,
)]
/// The response type for function [super::functions::req_pq_multi]
pub enum ResPQ {
    /// Response
    ResPQ(Box<resPQ>),
}

#[derive(
    Clone, Debug, tdlib_rs_impl::OptDeserialize, tdlib_rs_impl::From1, tdlib_rs_impl::Serialize,
)]
/// The inner data in [Step 4](https://core.telegram.org/mtproto/auth_key#presenting-proof-of-work-server-authentication).
pub enum P_Q_inner_data {
    /// Used to create permanent authorization keys.
    P_Q_inner_data_dc(Box<p_q_inner_data_dc>),
    /// Used to create temporary authorization keys.
    P_Q_inner_data_temp_dc(Box<p_q_inner_data_temp_dc>),
}

impl P_Q_inner_data {
    /// Create a new instance from [resPQ].
    pub fn new(v: &resPQ, expired_in: Option<i32>) -> Result<Self, FactorizeError> {
        if let Some(expired_in) = expired_in {
            let mut p = p_q_inner_data_temp_dc::try_from(v)?;
            p.expires_in = expired_in;
            Ok(Self::P_Q_inner_data_temp_dc(Box::new(p)))
        } else {
            Ok(Self::P_Q_inner_data_dc(Box::new(
                p_q_inner_data_dc::try_from(v)?,
            )))
        }
    }
}

#[derive(Clone, Debug, tdlib_rs_impl::From1, tdlib_rs_impl::Serialize)]
pub enum Server_DH_Params {
    Ok(Box<server_DH_params_ok>),
    Failed(i32),
}

impl Deserialize for Server_DH_Params {
    type Error = DeserializeError;
    fn deserialize<T: std::io::Read>(data: &mut T) -> Result<Self, Self::Error> {
        let type_id = u32::deserialize(data)?;
        if type_id == server_DH_params_ok::type_id2() {
            Ok(Self::Ok(Box::new(server_DH_params_ok::deserialize(data)?)))
        } else {
            Ok(Self::Failed(type_id as i32))
        }
    }
}

#[derive(
    Clone, Debug, tdlib_rs_impl::OptDeserialize, tdlib_rs_impl::From1, tdlib_rs_impl::Serialize,
)]
pub enum Server_DH_Inner_Data {
    Boxed(Box<server_DH_inner_data>),
}
