pub fn check_magic_header(magic: &[u8], data: &[u8]) -> bool {
    if data.len() < magic.len() {
        return false;
    }
    magic.iter().enumerate().all(|(i, byte)| *byte == data[i])
}
