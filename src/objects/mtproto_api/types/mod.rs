/// RSA public key
mod rsa_public_key;

use super::constructors::*;
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
