pub fn to_vec(it: impl writeable::Writeable) -> Vec<u8> {
    it.write_to_string().into_owned().into_bytes()
}

pub fn try_to_vec<W>(it: &W) -> Result<Vec<u8>, W::Error>
where
    W: writeable::TryWriteable,
{
    it.try_write_to_string()
        .map(|it| it.into_owned().into_bytes())
        .map_err(|(e, _)| e)
}
