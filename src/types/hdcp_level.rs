use crate::{Error, ErrorKind, Result};
use std::fmt;
use std::str::{self, FromStr};

/// HDCP level.
///
/// See: [4.3.4.2. EXT-X-STREAM-INF]
///
/// [4.3.4.2. EXT-X-STREAM-INF]: https://tools.ietf.org/html/rfc8216#section-4.3.4.2
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HdcpLevel {
    Type0,
    None,
}

impl fmt::Display for HdcpLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HdcpLevel::Type0 => "TYPE-0".fmt(f),
            HdcpLevel::None => "NONE".fmt(f),
        }
    }
}

impl FromStr for HdcpLevel {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "TYPE-0" => Ok(HdcpLevel::Type0),
            "NONE" => Ok(HdcpLevel::None),
            _ => track_panic!(ErrorKind::InvalidInput, "Unknown HDCP level: {:?}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let level = HdcpLevel::Type0;
        assert_eq!(level.to_string(), "TYPE-0".to_string());

        let level = HdcpLevel::None;
        assert_eq!(level.to_string(), "NONE".to_string());
    }

    #[test]
    fn test_parse() {
        let level = HdcpLevel::Type0;
        assert_eq!(level, "TYPE-0".parse::<HdcpLevel>().unwrap());

        let level = HdcpLevel::None;
        assert_eq!(level, "NONE".parse::<HdcpLevel>().unwrap());
    }
}