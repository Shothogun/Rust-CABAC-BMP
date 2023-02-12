use std::vec;

#[repr(u8)]
#[derive(PartialEq)]
pub enum BitstreamMode {
    Write = 0,
    Read = 1,
    NumberOfModes = 2,
}

#[repr(u8)]
#[derive(PartialEq)]
pub enum BitstreamReadingStatus {
    NotStarted = 0,
    Reading = 1,
    Finished = 2,
}

pub struct Bitstream {
    pub data: Vec<u8>,
    pub num_buf8: u8,
    pub buf8: u8,
    pub bitstream_pointer: u64,
    pub bitstream_mode: BitstreamMode,
    pub reading_status: BitstreamReadingStatus,
    pub reading_num_valid_bits_last_byte: u8,
}

impl Bitstream {
    pub fn new() -> Self {
        Self {
            data: vec![0; 1024],
            num_buf8: 0,
            buf8: 0,
            bitstream_pointer: 0,
            reading_num_valid_bits_last_byte: 0,
            bitstream_mode: BitstreamMode::Write,
            reading_status: BitstreamReadingStatus::NotStarted,
        }
    }

    // pub fn new_from_bitstream(&mut self, bs2: Bitstream, nbits: u64) {
    //     if nbits <= bs2.number_of_remaining_bits() {
    //         self.data = vec![0; (8 * nbits).try_into().unwrap()];

    //         let bit: bool;

    //         for i in 0..nbits {
    //             bit = bs2.read_bit();
    //             self.write_bit(bit);

    //             self.change_mode_to_read();
    //         }
    //     } else {
    //         println!("Bitstream::Tried to cut more bits than I have.")
    //     }
    // }

    // pub fn new_from_file() {

    // }

    pub fn write_bit(&mut self, bit: bool) {
        self.buf8 <<= 1;
        self.buf8 |= bit as u8;

        self.num_buf8 += 1;

        if self.num_buf8 == 8 {
            self.data.push(self.buf8);

            self.buf8 = 0;
            self.num_buf8 = 0;
            self.bitstream_pointer += 1;
        }
    }

    pub fn merge(&mut self, bs: Bitstream) {
        let mut curr_byte: u8;
        let mut curr_bit: bool;

        for element in self.data.clone() {
            curr_byte = element as u8;
            for _ in 0..8 {
                curr_bit = (curr_byte & 0x80) == 0x80;
                curr_byte <<= 1;

                self.write_bit(curr_bit);
            }
        }

        if bs.num_buf8 > 0 {
            curr_byte = bs.buf8;
            curr_byte <<= 8 - bs.num_buf8;

            for _ in 0..bs.num_buf8 {
                curr_bit = (curr_byte & 0x80) == 0x80;
                curr_byte <<= 1;

                self.write_bit(curr_bit);
            }
        }
    }

    pub fn change_mode_to_read(&mut self) {
        if self.num_buf8 != 0 {
            let mut temp: u8 = self.buf8;
            temp <<= 8 - self.num_buf8;
            self.data.push(temp);
        }

        self.reading_num_valid_bits_last_byte = self.num_buf8;
        self.buf8 = self.data[0];
        self.num_buf8 = 8;
        self.bitstream_pointer = 1;
        self.reading_status = BitstreamReadingStatus::Reading;
        self.bitstream_mode = BitstreamMode::Read;
    }

    pub fn total_size(&self) -> u64 {
        if self.bitstream_mode == BitstreamMode::Write {
            return 8 * self.bitstream_pointer + self.num_buf8 as u64;
        } else {
            return 8 * (self.data.len() as u64 - 1) + self.reading_num_valid_bits_last_byte as u64;
        }
    }

    pub fn number_of_remaining_bits(&self) -> u64 {
        if self.bitstream_mode == BitstreamMode::Write {
            return 0;
        } else {
            let total: u64 =
                8 * (self.data.len() as u64 - 1) + self.reading_num_valid_bits_last_byte as u64;

            let read: u64 = if self.bitstream_pointer < self.data.len() as u64 {
                8 * (self.bitstream_pointer - 1) + 8 - self.num_buf8 as u64
            } else {
                8 * (self.bitstream_pointer - 1) + self.reading_num_valid_bits_last_byte as u64
                    - self.num_buf8 as u64
            };

            return total - read;
        }
    }

    pub fn read_bit(&mut self) -> bool {
        let mut bit: bool = false;

        if self.reading_status == BitstreamReadingStatus::Reading {
            bit = (self.buf8 & 0x80) == 0x80;

            self.buf8 <<= 1;

            self.num_buf8 -= 1;

            if self.num_buf8 == 0 {
                // Is there a next byte?
                if self.bitstream_pointer < (self.data.len() - 1) as u64 {
                    // Just grabs the next byte.
                    self.buf8 = self.data[self.bitstream_pointer as usize];
                    self.num_buf8 = 8;
                    self.bitstream_pointer += 1;
                } else if self.bitstream_pointer < self.data.len() as u64 {
                    // Gets the next byte.
                    self.buf8 = self.data[self.bitstream_pointer as usize];
                    // Adjusts the number of usable bits.
                    self.num_buf8 = self.reading_num_valid_bits_last_byte;
                    self.bitstream_pointer += 1;
                } else {
                    // This was the last bit.
                    self.reading_status = BitstreamReadingStatus::Finished;
                }
            }
        } else {
            println!(
                "Error - Attempting to read a {} bitstream.",
                if self.reading_status == BitstreamReadingStatus::NotStarted {
                    "NOT STARTED"
                } else {
                    "FINISHED"
                }
            );
        }
        return bit;
    }

    pub fn rewind(&mut self) {
        self.bitstream_pointer = 0;
    }

    pub fn write_number(&mut self, mut num: u64, mut n_bits: u64) {
        let curr_mask: u64 = 1 << (n_bits - 1);

        while n_bits != 0 {
            let curr_bit = num & curr_mask;
            self.write_bit(if curr_bit != 0 { true } else { false });

            num <<= 1;
            n_bits -= 1;
        }
    }

    pub fn read_number(&mut self, mut n_bits: u64) -> u64 {
        let mut num: u64 = 0;
        let valid_mask: u64 = (1 << n_bits) - 1;

        while n_bits != 0 {
            let curr_bit: bool = self.read_bit();

            num <<= 1;
            num += curr_bit as u64;
            n_bits -= 1;
        }

        num &= valid_mask;
        return num;
    }
}
