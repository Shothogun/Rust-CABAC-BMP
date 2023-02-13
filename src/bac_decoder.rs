use crate::{
    bac_codec::{self, ContextInfo},
    bitstream,
};
use bitstream_io::{BitQueue, BE};

// Binary Arithemtic Decoder data
pub struct BACDecoder {
    valid_bits_mask: u64,
    p_bitstream: bitstream::Bitstream,
    n_decoded: u64,
    curr_tag: u64,
    m: u64,
    msb_mask: u64,
    state: bac_codec::BACState,
}

impl BACDecoder {
    pub fn new(input_bitstream: bitstream::Bitstream, m: u64) -> Self {
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

    pub fn init(&mut self) {
        if self.m <= self.p_bitstream.number_of_remaining_bits() {
            let mut bit: bool;
            let mut i: u64 = self.m;

            while i > 0 {
                bit = self.p_bitstream.read_bit();
                self.curr_tag <<= 1;
                self.curr_tag <<= if bit {
                    self.curr_tag + 1
                } else {
                    self.curr_tag
                };
                i -= 1;
            }
        } else {
            panic!("BACDecoder::Tried to read more bits than I have.");
        }
    }

    pub fn decode_one_symbol(&mut self, c: ContextInfo) -> bool {
        let mps: bool = c.mps;
        let lps: bool = !(mps);
        let symbol: bool;

        let l0: u64 = self.state.ln;
        let u0: u64 = self.state.un;
        let mut l1: u64;
        let mut u1: u64;
        let tag_star: u64;
        let temp: u64;

        // Computes the tagStar
        tag_star = ((self.curr_tag - l0 + 1) * c.total_count - 1) / (u0 - l0 + 1);

        // Decodes the symbol.
        if tag_star < c.count_mps {
            symbol = mps;

            l1 = l0;

            temp = ((u0 - l0 + 1) * c.count_mps) / c.total_count;
            u1 = l0 + temp - 1;
        } else {
            symbol = lps;

            temp = ((u0 - l0 + 1) * c.count_mps) / c.total_count;
            l1 = l0 + temp;

            u1 = u0;
        }

        // Checks if I have to flush bits.
        let mut ok: bool = true;
        let mut msb_l: bool;
        let mut msb_u: bool;
        let mut bit: bool;

        while ok {
            msb_l = (l1 & self.msb_mask) != 0;
            msb_u = (u1 & self.msb_mask) != 0;

            if msb_l == msb_u {
                // If they are the same, I have to read a Bit from the bitstream.
                bit = self.p_bitstream.read_bit();

                // Then I have to flush it out of l1and u1.
                l1 = (l1 << 1) & self.valid_bits_mask;
                u1 = ((u1 << 1) & self.valid_bits_mask) + 1;

                // Updates the tag.
                self.curr_tag = (self.curr_tag << 1) & self.valid_bits_mask; // Flushes one bit out
                self.curr_tag += if bit { 1 } else { 0 }; // Inserts the bit read into the tag.
            } else {
                ok = false;
            }
        }

        // Updates the decoder.
        self.state.ln = l1;
        self.state.un = u1;
        self.n_decoded += 1;

        return symbol;
    }

    pub fn show_status(&self) {
        println!("DECODER STATUS: ");
        println!("ln            : {}", self.state.ln);
        println!("un            : {}", self.state.un);
        println!("Number of Decoded Symbols = {}", self.n_decoded);
        println!(
            "Number of bits in bitstream = {}",
            self.p_bitstream.number_of_remaining_bits()
        );
    }
}
