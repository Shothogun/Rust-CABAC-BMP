use bitstream_io::{BitQueue, BE};
use std::clone::Clone;

mod BACCodec {
    mod BACEncoder {
        // Binary Arithemtic Encoder data
        struct BACEncoder {
            m: u64,
            msb_mask: u64,
            valid_bits_mask: u64,
            state: BACState,
        }

        impl BACEncoder {
            fn CreateBACEncoder() -> BACEncoder {
                BACEncoder {
                    m: m,
                    msb_mask: 1 << (m - 1),
                    valid_bits_mask: (1 << m) - 1,
                    state: BACState {
                        ln: 0,
                        un: (1 << m) - 1,
                    },
                }
            }

            fn BACEncoderGetState(&self) -> BACState {
                enc.state.clone()
            }

            fn BACEncoderSetState(&self, lnNew: u64, unNew: u64) {
                enc.ln = lnNew;
                enc.un = unNew;
            }
        }
    }
}
