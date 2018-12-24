use failure::Error;
use std::fs::File;
use std::io::Write;
use crc::crc32;

enum RecordType {
    // Zero is reserved for preallocated files
    Zero = 0,
    Full = 1,
    First = 2,
    Middle = 3,
    Last = 4,
}

const BLOCK_SIZE: usize = 32768;

// Header is checksum (4 bytes), length (2 bytes), type (1 byte).
const HEADER_SIZE: usize = 4 + 2 + 1;

const MAX_RECORD_TYPE: RecordType = RecordType::Last;

const BLOCK_PADDING_BYTES: &[u8] = b"\x00\x00\x00\x00\x00\x00";

pub struct Writer {
    dest: File,
    block_offset: usize,
}

impl Writer {
    pub fn new(dest: File) -> Writer {
        return Writer {
            dest,
            block_offset: 0,
        }
    }

    pub fn add_record(&mut self, data: &[u8]) -> Result<(), Error> {
        let mut remain = data;
        let begin = true;
        while !remain.is_empty() {
            let available_block_size = BLOCK_SIZE - self.block_offset;
            assert!(available_block_size >= 0);

            // Switch to new block
            if available_block_size < HEADER_SIZE {
                // Fill the trailer (literal below relies on kHeaderSize being 7)
                assert_eq!(HEADER_SIZE, 7);

                if available_block_size > 0 {
                    self.dest.write(&BLOCK_PADDING_BYTES[0..available_block_size]);
                }
                self.block_offset = 0;
            }

            let available_data_size = BLOCK_SIZE - self.block_offset - HEADER_SIZE;

            // Invariant: we never leave < kHeaderSize bytes in a block.
            assert!(available_data_size >= 0);
            let fragment_length = if remain.len() < available_data_size { remain.len() } else { available_data_size };
            let end = fragment_length == remain.len();

            let record_type = Self::record_type(begin, end);
            self.emit_physical_record(record_type, &remain[0..fragment_length])?;

            remain = &remain[fragment_length..remain.len()];
        }
        panic!("")
    }

    fn record_type(begin: bool, end: bool) -> RecordType {
        if begin && end {
            return RecordType::Full;
        } else if begin {
            return RecordType::First;
        } else if end {
            return RecordType::Last;
        } else {
            return RecordType::Middle;
        }
    }

    fn emit_physical_record(&mut self, record_type: RecordType, record: &[u8]) -> Result<(), Error> {
        assert!(record.len() <= 0xffff);  // Must fit in two bytes

        let mut digest = crc32::Digest::new_with_initial(crc32::IEEE, record_type as u32);
        digest.write(record);

        panic!("")
    }
}