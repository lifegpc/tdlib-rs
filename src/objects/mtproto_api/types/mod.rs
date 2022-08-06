use super::constructors::*;

#[derive(Clone, Debug, tdlib_rs_impl::From1, tdlib_rs_impl::Serialize)]
/// The response type for function [super::functions::req_pq_multi]
pub enum ResPQ {
    /// Response
    ResPQ(Box<resPQ>),
}
