// Put u32 into buf in little endian
pub fn put_u32(value: u32, buf: &mut [u8]) {
    assert_eq!(buf.len(), 4);

    buf[0] = (value & 0xff) as u8;
    buf[1] = ((value >> 8) & 0xff) as u8;
    buf[2] = ((value >> 16) & 0xff) as u8;
    buf[3] = (value >> 24) as u8;
}

pub fn decode_u32(buf: &[u8]) -> u32 {
    assert_eq!(buf.len(), 4);

    buf[0] as u32
        | ((buf[1] as u32) << 8)
        | ((buf[2] as u32) << 16)
        | ((buf[3] as u32) << 24)
}

// Encode u32 into bytes in little endian
pub fn encode_u32(value: u32) -> Vec<u8> {
    let mut buf = vec![0u8; 4];
    put_u32(value, &mut buf);

    buf
}

// Put u16 into buf in little endian
pub fn put_u16(value: u16, buf: &mut [u8]) {
    assert_eq!(buf.len(), 2);

    buf[0] = (value & 0xff) as u8;
    buf[1] = (value >> 8) as u8;
}

pub fn decode_u16(buf: &[u8]) -> u16 {
    assert_eq!(buf.len(), 2);

    buf[0] as u16 | ((buf[1] as u16) << 8)
}

// Encode u16 into bytes in little endian
pub fn encode_u16(value: u16) -> Vec<u8> {
    let mut buf = vec![0u8; 2];
    put_u16(value, &mut buf);

    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_u16() {
        let bytes = encode_u16(0xAABB);
        assert_eq!(bytes, vec![0xBB, 0xAA]);
    }

    #[test]
    fn test_encode_u32() {
        let bytes = encode_u32(0xAABBCCDD);
        assert_eq!(bytes, vec![0xDD, 0xCC, 0xBB, 0xAA]);
    }

    #[test]
    fn test_decode_u16() {
        let value = decode_u16(&vec![0xAA, 0xBB]);
        assert_eq!(value, 0xBBAA);
    }

    #[test]
    fn test_decode_u32() {
        let value = decode_u32(&vec![0xAA, 0xBB, 0xCC, 0xDD]);
        assert_eq!(value, 0xDDCCBBAA);
    }
}