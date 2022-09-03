use crate::random::Random;

use openssl::bn::{BigNum, BigNumContext};
use std::ops::Deref;

/// returns (c + a * b) % pq
fn pq_add_mul(c: u64, a: u64, b: u64, pq: u64) -> u64 {
    let mut c = c;
    let mut b = b;
    let mut a = a;
    while b != 0 {
        if b & 1 != 0 {
            c += a;
            if c >= pq {
                c -= pq;
            }
        }
        a += a;
        if a >= pq {
            a -= pq;
        }
        b >>= 1;
    }
    return c;
}

fn pq_gcd(a: u64, b: u64) -> u64 {
    if a == 0 {
        return b;
    }
    let mut a = a;
    while (a & 1) == 0 {
        a >>= 1;
    }
    let mut b = b;
    loop {
        if a > b {
            a = (a - b) >> 1;
            while (a & 1) == 0 {
                a >>= 1;
            }
        } else if b > a {
            b = (b - a) >> 1;
            while (b & 1) == 0 {
                b >>= 1;
            }
        } else {
            return a;
        }
    }
}

fn pq_factorize1(pq: u64) -> u64 {
    if pq <= 2 || pq > (1 << 63) {
        return 1;
    }
    if (pq & 1) == 0 {
        return 2;
    }
    let mut g = 0u64;
    let mut i = 0;
    let mut iter = 0;
    let mut random = Random::new();
    while i < 3 || iter < 1000 {
        let q = (random.gen_u8_in_range(17, 32) as u64) % (pq - 1);
        let mut x = random.gen_u64() % (pq - 1) + 1;
        let mut y = x;
        let lim = 1 << (std::cmp::min(5, i) + 18);
        for j in 1..lim {
            iter += 1;
            x = pq_add_mul(q, x, x, pq);
            let z = if x < y { pq + x - y } else { x - y };
            g = pq_gcd(z, pq);
            if g != 1 {
                break;
            }
            if j & (j - 1) == 0 {
                y = x;
            }
        }
        if g > 1 && g < pq {
            break;
        }
        i += 1;
    }
    if g != 0 {
        let other = pq / g;
        if other < g {
            g = other;
        }
    }
    g
}

fn pq_factorize_big(pq: &[u8]) -> Result<Option<(Vec<u8>, Vec<u8>)>, openssl::error::ErrorStack> {
    let mut ctx = BigNumContext::new()?;
    let mut p = BigNum::new()?;
    let mut q = BigNum::new()?;
    let one = BigNum::from_u32(1)?;
    let pq = BigNum::from_slice(pq)?;
    let mut found = false;
    let mut i = 0;
    let mut iter = 0;
    let mut random = Random::new();
    while !found && (i < 3 || iter < 1000) {
        let t = random.gen_u32_in_range(17, 32);
        let mut a = BigNum::from_u32(random.gen_u32())?;
        let mut b = a.deref().to_owned()?;
        let lim = 1 << (i + 23);
        for j in 1..lim {
            iter += 1;
            let tmp = a.deref().to_owned()?;
            a.mod_mul(&tmp, &tmp, &pq, &mut ctx)?;
            a.add_word(t)?;
            if a >= pq {
                let mut tmp = BigNum::new()?;
                tmp.checked_sub(&a, &pq)?;
                a = tmp;
            }
            if a > b {
                q.checked_sub(&a, &b)?;
            } else {
                q.checked_sub(&b, &a)?;
            }
            p.gcd(&q, &pq, &mut ctx)?;
            if p != one {
                found = true;
                break;
            }
            if j & (j - 1) == 0 {
                b = a.deref().to_owned()?;
            }
        }
        i += 1;
    }
    if found {
        q.checked_div(&pq, &p, &mut ctx)?;
        if p > q {
            std::mem::swap(&mut p, &mut q);
        }
        Ok(Some((p.to_vec(), q.to_vec())))
    } else {
        Ok(None)
    }
}

pub fn pq_factorize(pqs: &[u8]) -> Result<Option<(Vec<u8>, Vec<u8>)>, openssl::error::ErrorStack> {
    let size = pqs.len();
    if size > 8 || (size == 8 && (pqs[0] & 128) != 0) {
        return pq_factorize_big(pqs);
    }
    let mut pq = 0u64;
    for i in 0..size {
        pq = (pq << 8) | (pqs[i] as u64);
    }
    let p = pq_factorize1(pq);
    if p == 0 || pq % p != 0 {
        Ok(None)
    } else {
        let mut rp = p.to_be_bytes().to_vec();
        while rp[0] == 0 {
            rp.remove(0);
        }
        let mut rq = (pq / p).to_be_bytes().to_vec();
        while rq[0] == 0 {
            rq.remove(0);
        }
        Ok(Some((rp, rq)))
    }
}
