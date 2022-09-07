use crate::dc_cache::{is_good_prime, PrimeCacheStatus, add_prime};
use crate::objects::traits::TypeId;
use bytes::BytesMut;
use openssl::bn::{BigNum, BigNumContext};

/// Origin: `server_DH_inner_data#b5890dba nonce:int128 server_nonce:int128 g:int dh_prime:string g_a:string server_time:int = Server_DH_inner_data;`
#[derive(Clone, Debug, tdlib_rs_impl::Deserialize, tdlib_rs_impl::Serialize)]
pub struct server_DH_inner_data {
    /// Value generated by client in [Step 1](https://core.telegram.org/mtproto/auth_key#dh-exchange-initiation).
    pub nonce: i128,
    /// Value received from server in [Step 2](https://core.telegram.org/mtproto/auth_key#dh-exchange-initiation).
    pub server_nonce: i128,
    pub g: i32,
    pub dh_prime: BytesMut,
    pub g_a: BytesMut,
    pub server_time: i32,
}

#[derive(Clone, Debug, derive_more::Display, derive_more::From)]
/// Error when checking `dh_prime`
pub enum CheckDhPrimeError {
    /// OpenSSL Error
    OpenSSLError(openssl::error::ErrorStack),
    /// String error
    String(String),
}

impl From<&str> for CheckDhPrimeError {
    fn from(s: &str) -> Self {
        Self::String(s.to_string())
    }
}

impl server_DH_inner_data {
    /// Check `dh_prime`
    pub fn check_dh_prime(&self) -> Result<(), CheckDhPrimeError> {
        let prime = BigNum::from_slice(&self.dh_prime)?;
        if prime.num_bits() != 2048 {
            return Err("dh_prime is not 2048 bits".into());
        }
        let n24 = BigNum::from_u32(24)?;
        let n23 = BigNum::from_u32(23)?;
        let n19 = BigNum::from_u32(19)?;
        let n8 = BigNum::from_u32(8)?;
        let n7 = BigNum::from_u32(7)?;
        let n6 = BigNum::from_u32(6)?;
        let n5 = BigNum::from_u32(5)?;
        let n4 = BigNum::from_u32(4)?;
        let n3 = BigNum::from_u32(3)?;
        let n2 = BigNum::from_u32(2)?;
        let n1 = BigNum::from_u32(1)?;
        let mut ctx = BigNumContext::new()?;
        let mod_ok = match self.g {
            2 => &prime % &n8 == n7,
            3 => &prime % &n3 == n2,
            4 => true,
            5 => {
                let mod_r = &prime % &n5;
                mod_r == n1 || mod_r == n4
            }
            6 => {
                let mod_r = &prime % &n24;
                mod_r == n19 || mod_r == n23
            }
            7 => {
                let mod_r = &prime % &n7;
                mod_r == n3 || mod_r == n5 || mod_r == n6
            }
            _ => false,
        };
        if !mod_ok {
            return Err("Bad prime mod 4g.".into());
        }
        match is_good_prime(&self.dh_prime) {
            PrimeCacheStatus::Good => Ok(()),
            PrimeCacheStatus::Bad => Err("p or (p - 1) / 2 is not a prime number.".into()),
            PrimeCacheStatus::Miss => {
                if !prime.is_prime(64, &mut ctx)? {
                    add_prime(&self.dh_prime, false);
                    return Err("p is not a prime number.".into());
                }
                let mut half_prime = prime.to_owned()?;
                half_prime.sub_word(1)?;
                half_prime.div_word(2)?;
                if !half_prime.is_prime(64, &mut ctx)? {
                    add_prime(&self.dh_prime, false);
                    return Err("p or (p - 1) / 2 is not a prime number.".into());
                }
                add_prime(&self.dh_prime, true);
                Ok(())
            }
        }
    }
}

impl TypeId for server_DH_inner_data {
    fn type_id2() -> u32 {
        0xb5890dba
    }
}
