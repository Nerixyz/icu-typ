pub fn to_vec(it: impl writeable::Writeable) -> Vec<u8> {
    it.write_to_string().into_owned().into_bytes()
}
