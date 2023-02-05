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
