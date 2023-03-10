use crate::bac_codec;
use crate::bitstream;
use std::clone::Clone;

// Binary Arithemtic Encoder data
#[derive(PartialEq, Debug)]
pub struct BACEncoder {
    pub m: u64,
    pub msb_mask: u64,
    pub valid_bits_mask: u64,
    pub state: bac_codec::BACState,
}

impl BACEncoder {
    pub fn new(m: u64) -> Self {
        Self {
            m: m,
            msb_mask: 1 << (m - 1),
            valid_bits_mask: (1 << m) - 1,
            state: bac_codec::BACState {
                ln: 0,
                un: (1 << m) - 1,
            },
        }
    }

    pub fn bacencoder_get_state(&self) -> bac_codec::BACState {
        self.state.clone()
    }

    pub fn bac_encoder_set_state(&mut self, ln_new: u64, un_new: u64) {
        self.state.ln = ln_new;
        self.state.un = un_new;
    }

    pub fn encode_one_symbol(
        &mut self,
        symbol: bool,
        c: bac_codec::ContextInfo,
        mut bitstream: bitstream::Bitstream,
    ) {
        let is_mps = symbol == c.mps;
        let bac_codec::BACState { ln: l0, un: u0 } = self.state.clone();
        let mut l1: u64;
        let mut u1: u64;
        let temp: u64;

        if is_mps {
            l1 = l0;
            temp = ((u0 - l0 + 1) * c.count_mps) / c.total_count;

            u1 = l0 + temp - 1;
        } else {
            temp = ((u0 - l0 + 1) * c.count_mps) / c.total_count;
            l1 = l0 + temp;

            u1 = u0;
        }

        // Checks if I have to flush bits.
        let mut msb_l: bool;
        let mut msb_u: bool;
        let mut ok: bool = true;
        while ok {
            msb_l = (l1 & self.msb_mask) != 0;
            msb_u = (u1 & self.msb_mask) != 0;

            if msb_l == msb_u {
                bitstream.write_bit(msb_l);
                l1 = (l1 << 1) & self.valid_bits_mask;
                u1 = ((u1 << 1) & self.valid_bits_mask) + 1;
            } else {
                ok = false;
            }
        }

        self.bac_encoder_set_state(l1, u1)
    }

    pub fn close_bitstream(&self, mut bitstream: bitstream::Bitstream) {
        let mut i: u64 = self.m;
        let mut l0: u64 = self.state.ln;
        let mut bit: bool;

        while i > 0 {
            bit = (l0 & self.msb_mask) != 0;
            l0 = (l0 << 1) & self.valid_bits_mask;
            bitstream.write_bit(bit);
            i -= 1;
        }
    }

    pub fn show_status(&self) {
        println!("ENCODER STATUS: ");
        println!("ln            : {}", self.state.ln);
        println!("un            : {}", self.state.un);
    }
}
