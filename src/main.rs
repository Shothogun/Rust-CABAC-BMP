pub mod bac_codec;
pub mod bac_decoder;
pub mod bac_encoder;
pub mod bitstream;

fn main() {
    let encoder: bac_encoder::BACEncoder = bac_encoder::BACEncoder::new(4);
    println!("Hello, world!");
    encoder.show_status();
}

#[test]
fn create_properly_an_encoder() {
    let encoder: bac_encoder::BACEncoder = bac_encoder::BACEncoder::new(4);
    let x = bac_encoder::BACEncoder {
        m: 4,
        msb_mask: 8,
        valid_bits_mask: 15,
        state: bac_codec::BACState { ln: 0, un: 15 },
    };
    assert_eq!(encoder, x);
}
