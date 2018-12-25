use crate::errors::FileError;
use crate::coding;
use failure::Error;
use std::fs::File;
use std::io::{Cursor, Read, Write, BufWriter, BufReader};
use crc::{crc32, Hasher32};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

#[derive(Copy, Clone)]
enum RecordType {
    // Zero is reserved for preallocated files
    Zero = 0,
    Full = 1,
    First = 2,
    Middle = 3,
    Last = 4,
}

const MASK_DELTA: u32 = 0xa282ead8;

const BLOCK_SIZE: usize = 32768;

// Header is checksum (4 bytes), length (2 bytes), type (1 byte).
const HEADER_SIZE: usize = 4 + 2 + 1;

const MAX_RECORD_TYPE: RecordType = RecordType::Last;

const BLOCK_PADDING_BYTES: &[u8] = b"\x00\x00\x00\x00\x00\x00";

#[derive(Debug)]
pub struct Writer {
    inner: BufWriter<File>,
    block_offset: usize,
}

impl Writer {
    pub fn new(dest: File) -> Writer {
        Writer {
            inner: BufWriter::new(dest),
            block_offset: 0,
        }
    }

    pub fn add_record(&mut self, data: &[u8]) -> Result<(), Error> {
        let mut remain = data;
        let mut begin = true;
        while !remain.is_empty() {
            let available_block_size = BLOCK_SIZE - self.block_offset;
            assert!(available_block_size >= 0);

            // Switch to new block
            if available_block_size < HEADER_SIZE {
                // Fill the trailer (literal below relies on kHeaderSize being 7)
                assert_eq!(HEADER_SIZE, 7);

                if available_block_size > 0 {
                    self.inner.write(&BLOCK_PADDING_BYTES[0..available_block_size]);
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
            begin = false;
            self.block_offset += HEADER_SIZE + fragment_length;
        }
        Ok(())
    }

    fn record_type(begin: bool, end: bool) -> RecordType {
        if begin && end {
            RecordType::Full
        } else if begin {
            RecordType::First
        } else if end {
            RecordType::Last
        } else {
            RecordType::Middle
        }
    }

    fn emit_physical_record(&mut self, record_type: RecordType, record: &[u8]) -> Result<(), Error> {
        assert!(record.len() <= 0xffff);  // Must fit in two bytes
        assert!(self.block_offset + HEADER_SIZE + record.len() <= BLOCK_SIZE);

        self.emit_header(record_type, record)?;

        self.inner.write(record)?;
        self.inner.flush()?;

        Ok(())
    }

    fn emit_header(&mut self, record_type: RecordType, record: &[u8]) -> Result<(), Error> {
        let mut digest = crc32::Digest::new_with_initial(crc32::IEEE, record_type as u32);
        digest.write(record);
        let crc = Self::mask_crc(digest.sum32());

        let mut header = Vec::with_capacity(HEADER_SIZE);
        {
            let mut cursor = Cursor::new(&mut header);
            cursor.write_u32::<LittleEndian>(crc)?;
            cursor.write_u16::<LittleEndian>(record.len() as u16)?;
            cursor.write_u8(record_type as u8)?;
        }

        self.inner.write(&header)?;

        Ok(())
    }

    fn mask_crc(crc: u32) -> u32 {
        // Rotate right by 15 bits and add a constant.
        ((crc >> 15) | (crc << 17)) + MASK_DELTA
    }
}

#[derive(Debug)]
pub struct Reader {
    reader: BufReader<File>,
    block_offset: usize,
    current_block: Vec<u8>,
}

impl Reader {
    pub fn new(dest: File) -> Reader {
        Reader {
            reader: BufReader::new(dest),
            block_offset: 0,
            current_block: Vec::with_capacity(BLOCK_SIZE),
        }
    }

    pub fn read_record(&mut self) -> Result<Vec<u8>, Error> {
        loop {
            if self.block_offset == 0 {
                self.reader.read(&mut self.current_block)?;

                // Ensure read the whole block
                assert_eq!(self.current_block.len(), BLOCK_SIZE);
            }

            let header = &self.current_block[self.block_offset..self.block_offset+HEADER_SIZE];
            let mut cursor = Cursor::new(header);
            let expected_crc = cursor.read_u32::<LittleEndian>()?;
            let record_length = cursor.read_u16::<LittleEndian>()? as usize;
            let record_type = cursor.read_u8()?;

            let record = &self.current_block[self.block_offset+HEADER_SIZE..self.block_offset+HEADER_SIZE+record_length];

            let mut digest = crc32::Digest::new_with_initial(crc32::IEEE, record_type as u32);
            digest.write(record);

            let actual_crc = digest.sum32();

            if actual_crc != expected_crc {
                let e = Error::from(FileError::Corrupted("invalid checksum".to_owned()));
                return Err(e);
            }
        }
    }

    fn unmask_crc(crc: u32) -> u32 {
        let c = crc - MASK_DELTA;
        (c << 15) | (c >> 17)
    }
}