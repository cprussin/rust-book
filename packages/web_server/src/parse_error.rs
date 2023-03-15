#[derive(Debug)]
pub enum ParseError {
    NoData,
    InvalidStartLine,
    InvalidVersion,
    InvalidMethod,
    InvalidHeader,
}
