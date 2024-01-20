// for debug purposes only
pub(crate) fn bytes_to_string(bytes: &[u8]) -> String {
    if let Ok(string) = std::str::from_utf8(bytes) {
        return string.to_owned();
    }
    format!("{:?}", bytes)
}
