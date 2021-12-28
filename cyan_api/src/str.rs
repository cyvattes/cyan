pub(crate) fn from_utf8(v: &[u8]) -> &str {
    std::str::from_utf8(v).expect("Invalid Request")
}