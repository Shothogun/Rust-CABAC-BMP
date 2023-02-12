use crate::bac_codec;
use bitstream_io::{BitQueue, BE};

// Binary Arithemtic Decoder data
pub struct BACDecoder {
    valid_bits_mask: u64,
    p_bitstream: BitQueue<BE, u64>,
    n_decoded: u64,
    curr_tag: u64,
    m: u64,
    msb_mask: u64,
    state: bac_codec::BACState,
}

impl BACDecoder {
    pub fn new(input_bitstream: BitQueue<BE, u64>, m: u64) -> Self {
        Self {
            m: m,
            msb_mask: 1 << (m - 1),
            valid_bits_mask: (1 << m) - 1,
            p_bitstream: input_bitstream,
            curr_tag: 0,
            n_decoded: 0,
            state: bac_codec::BACState {
                ln: 0,
                un: (1 << m) - 1,
            },
        }
    }

    // pub fn init(&mut self){
    //     if(self.m <= )
    // }
}
