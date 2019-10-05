use std::fmt;
use std::str::FromStr;

use crate::attribute::AttributePairs;
use crate::tags::ExtXKey;
use crate::types::{ByteRange, ProtocolVersion};
use crate::utils::{quote, tag, unquote};
use crate::{Encrypted, Error, RequiredVersion};

/// # [4.4.2.5. EXT-X-MAP]
/// The [`ExtXMap`] tag specifies how to obtain the Media Initialization
/// Section, required to parse the applicable [Media Segment]s.
///
/// [Media Segment]: crate::MediaSegment
/// [4.4.2.5. EXT-X-MAP]:
/// https://tools.ietf.org/html/draft-pantos-hls-rfc8216bis-04#section-4.4.2.5
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExtXMap {
    uri: String,
    range: Option<ByteRange>,
    keys: Vec<ExtXKey>,
}

impl ExtXMap {
    pub(crate) const PREFIX: &'static str = "#EXT-X-MAP:";

    /// Makes a new [`ExtXMap`] tag.
    pub fn new<T: ToString>(uri: T) -> Self {
        Self {
            uri: uri.to_string(),
            range: None,
            keys: vec![],
        }
    }

    /// Makes a new [`ExtXMap`] tag with the given range.
    pub fn with_range<T: ToString>(uri: T, range: ByteRange) -> Self {
        Self {
            uri: uri.to_string(),
            range: Some(range),
            keys: vec![],
        }
    }

    /// Returns the `URI` that identifies a resource,
    /// that contains the media initialization section.
    pub const fn uri(&self) -> &String { &self.uri }

    /// Returns the range of the media initialization section.
    pub const fn range(&self) -> Option<ByteRange> { self.range }
}

impl Encrypted for ExtXMap {
    fn keys(&self) -> &Vec<ExtXKey> { &self.keys }

    fn keys_mut(&mut self) -> &mut Vec<ExtXKey> { &mut self.keys }
}

/// This tag requires [`ProtocolVersion::V6`].
impl RequiredVersion for ExtXMap {
    fn required_version(&self) -> ProtocolVersion { ProtocolVersion::V6 }
}

impl fmt::Display for ExtXMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::PREFIX)?;
        write!(f, "URI={}", quote(&self.uri))?;

        if let Some(value) = &self.range {
            write!(f, ",BYTERANGE={}", quote(value))?;
        }

        Ok(())
    }
}

impl FromStr for ExtXMap {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = tag(input, Self::PREFIX)?;

        let mut uri = None;
        let mut range = None;

        for (key, value) in input.parse::<AttributePairs>()? {
            match key.as_str() {
                "URI" => uri = Some(unquote(value)),
                "BYTERANGE" => {
                    range = Some((unquote(value).parse())?);
                }
                _ => {
                    // [6.3.1. General Client Responsibilities]
                    // > ignore any attribute/value pair with an unrecognized
                    // AttributeName.
                }
            }
        }

        let uri = uri.ok_or_else(|| Error::missing_value("EXT-X-URI"))?;
        Ok(Self {
            uri,
            range,
            keys: vec![],
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(
            ExtXMap::new("foo").to_string(),
            "#EXT-X-MAP:URI=\"foo\"".to_string(),
        );

        assert_eq!(
            ExtXMap::with_range("foo", ByteRange::new(9, Some(2))).to_string(),
            "#EXT-X-MAP:URI=\"foo\",BYTERANGE=\"9@2\"".to_string(),
        );
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            ExtXMap::new("foo"),
            "#EXT-X-MAP:URI=\"foo\"".parse().unwrap()
        );

        assert_eq!(
            ExtXMap::with_range("foo", ByteRange::new(9, Some(2))),
            "#EXT-X-MAP:URI=\"foo\",BYTERANGE=\"9@2\"".parse().unwrap()
        );
    }

    #[test]
    fn test_required_version() {
        assert_eq!(ExtXMap::new("foo").required_version(), ProtocolVersion::V6);
        assert_eq!(
            ExtXMap::with_range("foo", ByteRange::new(9, Some(2))).required_version(),
            ProtocolVersion::V6
        );
    }
}
