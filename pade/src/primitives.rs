use crate::{PadeDecode, PadeDecodeError, PadeEncode};
use alloy_sol_types::SolValue;

/// Uses the default alloy `abi_encode_packed` to PADE-encode this type.  We
/// share many primitives with Alloy so this makes it simple to implement the
/// standard encoding for them.  This macro is only meant to run here, so we
/// don't have to worry about it being externally sound
macro_rules! use_alloy_default {
    ($( $x:ty ), *) => {
        $(
            impl PadeEncode for $x {
                fn pade_encode(&self) -> Vec<u8> {
                    self.abi_encode_packed()
                }
            }
        )*
    };
}

macro_rules! prim_decode {
    ($( $x:ty ), *) => {
        $(
            impl PadeDecode for $x {
                fn pade_decode(buf: &mut &[u8], _: Option<u8>) -> Result<Self, PadeDecodeError>
                where
                    Self: Sized
                {
                    const BYTES : usize  = <$x>::BITS as usize / 8usize;
                    let mut con_buf = [0u8; BYTES];

                    for (i, con) in con_buf.iter_mut().enumerate().take(BYTES) {

                        let Some(next) = buf.get(i) else { return Err(PadeDecodeError::InvalidSize) };
                        *con = *next;
                    }

                    let res = <$x>::from_be_bytes(con_buf);
                    *buf = &buf[BYTES..];
                    Ok(res)
                }

                fn pade_decode_with_width(buf: &mut &[u8], size: usize, _: Option<u8>) -> Result<Self, PadeDecodeError>
                where
                    Self: Sized
                {
                    const BYTES: usize = <$x>::BITS as usize / 8usize;

                    if size > BYTES {
                        return Err(PadeDecodeError::IncorrectWidth)
                    }
                    // item size in bytes vs given rep.
                    let padding_offset = BYTES - size;

                    if buf.len() < size {
                        return Err(PadeDecodeError::InvalidSize)
                    }

                    // the actual size
                    let subslice = &buf[..size];

                    let mut con_buf = [0u8; BYTES];
                    for i in 0..size {
                        let Some(next) = subslice.get(i) else {
                            eprintln!("subslice.get() failed");
                            return Err(PadeDecodeError::InvalidSize)
                        };

                        con_buf[i + padding_offset] = *next;
                    }

                    let res = <$x>::from_be_bytes(con_buf);
                    *buf = &buf[size..];

                    Ok(res)
                }
            }
        )*
    };
}

prim_decode!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

use_alloy_default!(u16, u32, u64, u128, i8, i16, i32, i64, i128);

impl PadeEncode for u8 {
    fn pade_encode(&self) -> Vec<u8> {
        vec![*self]
    }
}

#[cfg(feature = "alloy")]
mod alloy_specific {

    use super::*;
    use alloy::{
        primitives::{
            Address, Bytes, FixedBytes, Signature, U160, U256, aliases::I24, normalize_v,
        },
        sol_types::SolValue,
    };

    use_alloy_default!(I24, U256, U160, Address, FixedBytes<32>);

    prim_decode!(I24, U160, U256);

    impl PadeDecode for Address {
        fn pade_decode(buf: &mut &[u8], _: Option<u8>) -> Result<Self, PadeDecodeError>
        where
            Self: Sized,
        {
            const BYTES: usize = 160 / 8usize;
            let mut con_buf = [0u8; BYTES];

            for (i, con) in con_buf.iter_mut().enumerate().take(BYTES) {
                let Some(next) = buf.get(i) else {
                    return Err(PadeDecodeError::InvalidSize);
                };
                *con = *next;
            }
            let res = Address::from_slice(&con_buf);
            *buf = &buf[BYTES..];
            Ok(res)
        }

        fn pade_decode_with_width(
            buf: &mut &[u8],
            size: usize,
            _: Option<u8>,
        ) -> Result<Self, PadeDecodeError>
        where
            Self: Sized,
        {
            const BYTES: usize = 160 / 8usize;
            // grab the padding amount

            if size > BYTES {
                return Err(PadeDecodeError::IncorrectWidth);
            }

            if buf.len() < size {
                return Err(PadeDecodeError::InvalidSize);
            }

            let offset = size - BYTES;
            let subslice = &buf[offset..size];

            let mut con_buf = [0u8; BYTES];
            for (i, con) in con_buf.iter_mut().enumerate().take(BYTES) {
                let Some(next) = subslice.get(i) else {
                    return Err(PadeDecodeError::InvalidSize);
                };
                *con = *next;
            }

            let res = Address::from_slice(&con_buf);
            *buf = &buf[size..];

            Ok(res)
        }
    }

    impl PadeDecode for Bytes {
        fn pade_decode(buf: &mut &[u8], _: Option<u8>) -> Result<Self, PadeDecodeError>
        where
            Self: Sized,
        {
            let res: Vec<u8> = PadeDecode::pade_decode(buf, None)?;
            Ok(Bytes::copy_from_slice(&res))
        }

        fn pade_decode_with_width(
            _: &mut &[u8],
            _: usize,
            _: Option<u8>,
        ) -> Result<Self, PadeDecodeError>
        where
            Self: Sized,
        {
            unreachable!()
        }
    }

    impl PadeEncode for Bytes {
        fn pade_encode(&self) -> Vec<u8> {
            let bytes = self.to_vec();
            let len = bytes.len().to_be_bytes();

            [vec![len[5], len[6], len[7]], bytes].concat()
        }
    }

    // Custom impl for Signature which needs validation
    impl PadeEncode for Signature {
        fn pade_encode(&self) -> Vec<u8> {
            let mut sig = [0u8; 65];
            sig[0] = self.v() as u8;
            sig[1..33].copy_from_slice(&self.r().to_be_bytes::<32>());
            sig[33..65].copy_from_slice(&self.s().to_be_bytes::<32>());
            sig.to_vec()
        }
    }

    impl PadeDecode for Signature {
        fn pade_decode(buf: &mut &[u8], _: Option<u8>) -> Result<Self, PadeDecodeError>
        where
            Self: Sized,
        {
            if buf.len() < 65 {
                return Err(PadeDecodeError::InvalidSize);
            }

            let bytes = &buf[0..65];
            let v = bytes[0];
            let r = U256::from_be_slice(&bytes[1..33]);
            let s = U256::from_be_slice(&bytes[33..65]);

            *buf = &buf[65..];

            Ok(Signature::new(r, s, normalize_v(v as u64).unwrap()))
        }

        fn pade_decode_with_width(
            _: &mut &[u8],
            _: usize,
            _: Option<u8>,
        ) -> Result<Self, PadeDecodeError>
        where
            Self: Sized,
        {
            unreachable!()
        }
    }

    impl PadeDecode for FixedBytes<32> {
        fn pade_decode(buf: &mut &[u8], _: Option<u8>) -> Result<Self, PadeDecodeError>
        where
            Self: Sized,
        {
            if buf.len() < 32 {
                return Err(PadeDecodeError::InvalidSize);
            }

            let res: [u8; 32] = PadeDecode::pade_decode(buf, None)?;
            Ok(FixedBytes::from_slice(&res))
        }

        fn pade_decode_with_width(
            _: &mut &[u8],
            _: usize,
            _: Option<u8>,
        ) -> Result<Self, PadeDecodeError>
        where
            Self: Sized,
        {
            unreachable!()
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::PadeEncode;

    #[test]
    fn implemented_pade() {
        let tim = 128_u128;
        println!("{:?}", tim.pade_header_bits());
    }

    #[cfg(feature = "alloy")]
    #[test]
    fn encodes_and_decodes_signature() {
        use alloy::{
            primitives::FixedBytes,
            signers::{SignerSync, local::LocalSigner},
        };

        use crate::PadeDecode;

        let signer = LocalSigner::random();
        let hash = FixedBytes::<32>::default();
        let sig = signer.sign_hash_sync(&hash).unwrap();
        let encoded = sig.pade_encode();
        let decoded_sig =
            alloy::primitives::Signature::pade_decode(&mut encoded.as_slice(), None).unwrap();
        assert_eq!(sig, decoded_sig);
    }
}
