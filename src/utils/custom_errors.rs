#[derive(Debug)]
struct JsonParseError;

impl std::fmt::Display for JsonParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JSON parsing error")
    }
}
impl std::error::Error for JsonParseError {}
