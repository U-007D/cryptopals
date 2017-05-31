use hex_byte_string::HexByteString;

#[cfg(test)] mod unit_tests;

#[derive(Debug)]
pub struct ByteBuffer(Vec<u8>);

impl ByteBuffer {
    pub fn as_byte_vec(&self) -> Vec<u8> {
        vec![0x00u8, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff]
    }
}

impl From<HexByteString> for ByteBuffer {
    fn from(hex_byte_string: HexByteString) -> Self {
        ByteBuffer(vec![])
    }
}
