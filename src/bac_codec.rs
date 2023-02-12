// Binary Arithemtic Codec State
#[derive(PartialEq)]
#[derive(Debug)]

pub struct BACState {
    pub ln: u64,
    pub un: u64,
}

impl Clone for BACState {
    fn clone(&self) -> BACState {
        BACState {
            ln: self.ln,
            un: self.un,
        }
    }
}
