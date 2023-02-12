use std::fs;
use std::fs::File;
use std::io::{Error, Write};
use std::path::Path;
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
            data: Vec::new(),
            num_buf8: 0,
            buf8: 0,
            bitstream_pointer: 0,
            reading_num_valid_bits_last_byte: 0,
            bitstream_mode: BitstreamMode::Write,
            reading_status: BitstreamReadingStatus::NotStarted,
        }
    }

    pub fn new_from_bitstream(&mut self, mut bs2: Bitstream, nbits: u64) {
        if nbits <= bs2.number_of_remaining_bits() {
            self.data = vec![0; (8 * nbits).try_into().unwrap()];

            let mut bit: bool;

            for _ in 0..nbits {
                bit = bs2.read_bit();
                self.write_bit(bit);

                self.change_mode_to_read();
            }
        } else {
            panic!("Bitstream::Tried to cut more bits than I have.")
        }
    }

    pub fn read_from_file(&mut self, file_path: String) {
        let file = fs::read(file_path);
        let mut content: Vec<u8>;

        match file {
            Ok(a) => content = a.clone(),
            Err(b) => panic!("Erro ao abrir arquivo! {}", b),
        }

        if content[0] & 0xF0 == 0xE0 {
            if content.len() == 1 {
                panic!("The bitstream has only one byte (the header).");
            } else {
                self.reading_num_valid_bits_last_byte = content[0] & 0x0F;
                self.data = content.split_off(1);
            }
        } else {
            panic!("Header 0xE_ not matching!");
        }
    }

    pub fn flush_to_file(&mut self, file_path: String) {
        let path: &Path = Path::new(&file_path);
        let file: Result<File, Error>;
        if path.exists() {
            file = File::create(file_path);
        } else {
            file = File::open(file_path);
        }

        match file {
            Err(err) => panic!("Erro ao abrir o arquivo! {}", err),
            Ok(mut f) => {
                // Computes and writes the first byte
                let num_valid_bits_in_last_byte: u8 =
                    if self.num_buf8 == 0 { 8 } else { self.num_buf8 };
                let first_byte: u8 = 0xE0 as u8 | num_valid_bits_in_last_byte as u8;

                match f.write(&[first_byte]) {
                    Err(err) => panic!("Erro na escrita do primeiro byte! {}", err),
                    _ => (),
                };

                let mut i: usize = 0;
                while i < self.data.len() {
                    match f.write(&[self.data[i]]) {
                        Err(err) => panic!("Erro na escrita do byte {}! {}", i, err),
                        _ => (),
                    };
                    i += 1;
                }
                // Writes the data that is already packed to 8 bits.

                if self.num_buf8 > 0 {
                    // Computes and writes the last byte.
                    let mut last_byte: u8 = self.buf8;
                    let mut i: u8 = 8 - self.num_buf8;
                    while i != 0 {
                        last_byte <<= 1;
                        i -= 1;
                    }

                    match f.write(&[last_byte]) {
                        Err(err) => panic!("Erro na escrita do último byte! {}", err),
                        _ => (),
                    };
                }
            }
        }
    }

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

#[test]
fn bitstream_write_and_read_bit() {
    let mut bs: Bitstream = Bitstream::new();
    bs.write_bit(true);
    bs.write_bit(false);
    bs.write_bit(true);
    bs.write_bit(true);
    bs.write_bit(true);
    bs.write_bit(false);
    bs.write_bit(false);
    bs.write_bit(true);

    bs.change_mode_to_read();

    assert_eq!(true, bs.read_bit());
    assert_eq!(false, bs.read_bit());
    assert_eq!(true, bs.read_bit());
    assert_eq!(true, bs.read_bit());
    assert_eq!(true, bs.read_bit());
    assert_eq!(false, bs.read_bit());
    assert_eq!(false, bs.read_bit());
    assert_eq!(true, bs.read_bit());
}
