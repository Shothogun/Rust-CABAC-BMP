mod BACCodec {
    // Binary Arithemtic Codec State
    struct BACState {
        ln: u64,
        un: u64,
    }

    impl Clone for BACState {
        fn clone(&self) -> BACState {
            BACState {
                ln: self.ln,
                un: self.un,
            }
        }
    }
}
