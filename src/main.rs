use bitstream_io::{BitQueue, BE};

// Binary Arithemtic Codec State
struct BACState {
    ln: u64,
    un: u64,
}

// Binary Arithemtic Decoder data
struct BACDecoder {
    valid_bits_mask: u64,
    p_bitstream: BitQueue<BE, u64>,
    n_decoded: u64,
    curr_tag: u64,
    m: u64,
    msb_mask: u64,
    state: BACState,
}

// Binary Arithemtic Encoder data
struct BACEncoder {
    m: u64,
    msb_mask: u64,
    valid_bits_mask: u64,
    state: BACState,
}

fn main() {
    println!("Hello, world!");
}
