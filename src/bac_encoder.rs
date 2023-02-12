use crate::bac_codec;
use bitstream_io::{BitQueue, BE};
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
}
