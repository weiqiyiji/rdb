// Encode value into bytes in little endian
pub fn encode_u32(value: u32) -> Vec<u8> {
    let mut buf = vec![0u8; 4];
    buf[0] = (value & 0xff) as u8;
    buf[1] = ((value >> 8) & 0xff) as u8;
    buf[2] = ((value >> 16) & 0xff) as u8;
    buf[3] = ((value >> 24) & 0xff) as u8;

    return buf;
}

#[cfg(test)]
mod tests {

}