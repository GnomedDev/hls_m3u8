use crate::types::ProtocolVersion;
use crate::{Error, ErrorKind, Result};
use std::fmt;
use std::str::FromStr;

/// [4.3.1.2. EXT-X-VERSION]
///
/// [4.3.1.2. EXT-X-VERSION]: https://tools.ietf.org/html/rfc8216#section-4.3.1.2
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExtXVersion {
    version: ProtocolVersion,
}

impl ExtXVersion {
    pub(crate) const PREFIX: &'static str = "#EXT-X-VERSION:";

    /// Makes a new `ExtXVersion` tag.
    pub fn new(version: ProtocolVersion) -> Self {
        ExtXVersion { version }
    }

    /// Returns the protocol compatibility version of the playlist containing this tag.
    pub fn version(self) -> ProtocolVersion {
        self.version
    }

    /// Returns the protocol compatibility version that this tag requires.
    pub fn requires_version(self) -> ProtocolVersion {
        ProtocolVersion::V1
    }
}

impl fmt::Display for ExtXVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", Self::PREFIX, self.version)
    }
}

impl FromStr for ExtXVersion {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        track_assert!(s.starts_with(Self::PREFIX), ErrorKind::InvalidInput);
        let suffix = s.split_at(Self::PREFIX.len()).1;
        let version = track!(suffix.parse())?;
        Ok(ExtXVersion { version })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ext_x_version() {
        let tag = ExtXVersion::new(ProtocolVersion::V6);
        assert_eq!("#EXT-X-VERSION:6".parse().ok(), Some(tag));
        assert_eq!(tag.to_string(), "#EXT-X-VERSION:6");
        assert_eq!(tag.version(), ProtocolVersion::V6);
        assert_eq!(tag.requires_version(), ProtocolVersion::V1);
    }
}
