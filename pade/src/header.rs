use std::cmp::min;

use bitvec::{order::Lsb0, vec::BitVec, view::BitView};

#[derive(Debug, Default)]
pub struct HeaderBits {
    inner: BitVec<u8, Lsb0>
}

impl HeaderBits {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_bits(&mut self, bytes: &[u8], bits: usize) {
        // 0 is fine, but we can never take more length than we have bytes
        let bits_to_take = min(bits, bytes.len() * 8);

        let nview = BitView::view_bits::<Lsb0>(bytes)
            .split_at(bits_to_take)
            .0
            .to_bitvec();
        self.inner.extend_from_bitslice(nview.as_bitslice());
    }

    pub fn into_vec(self) -> Vec<u8> {
        self.inner.into_vec()
    }
}

#[cfg(test)]
mod test {
    use super::HeaderBits;

    #[test]
    fn can_be_constructed() {
        let _h = HeaderBits::new();
    }

    #[test]
    fn empty_struct_outputs_zero() {
        let h = HeaderBits::new();
        let v = h.into_vec();
        assert_eq!(v.len(), 0, "Byte vector for empty header not equal to zero");

        let mut h2 = HeaderBits::new();
        h2.add_bits(&[13_u8], 0);
        let v2 = h2.into_vec();
        assert_eq!(v2.len(), 0, "Byte vector not empty after adding zero bits");
    }

    #[test]
    fn will_not_oversize_add() {
        let mut h = HeaderBits::new();
        h.add_bits(&[u8::MAX], 12);
        assert_eq!(h.inner.len(), 8, "Too many bits taken from only one byte");
        h.add_bits(&[u8::MAX, u8::MAX], 20);
        assert_eq!(h.inner.len(), 24, "Too many bits taken from only one byte");
        let v = h.into_vec();
        assert_eq!(v.len(), 3, "Too many bytes generated from a bit over-consumption");
        assert!(v.iter().all(|x| *x == u8::MAX), "Invalid values in bit output")
    }
}
