// Binary Arithemtic Codec State
#[derive(PartialEq, Debug)]

pub struct BACState {
    pub ln: u64,
    pub un: u64,
}

pub struct ContextInfo {
    pub mps: bool,
    pub countMPS: u64,
    pub totalCount: u64,
}

impl Clone for BACState {
    fn clone(&self) -> BACState {
        BACState {
            ln: self.ln,
            un: self.un,
        }
    }
}
