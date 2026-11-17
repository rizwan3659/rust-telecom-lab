use std::fmt;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct CommandFlags(u8);

impl CommandFlags {
    pub const REQUEST: u8 = 0x80;
    pub const PROXIABLE: u8 = 0x40;
    pub const ERROR: u8 = 0x20;
    pub const RETRANSMITTED: u8 = 0x10;

    pub fn new(bits: u8) -> Self {
        Self(bits & 0xF0)
    }

    pub fn is_request(&self) -> bool {
        (self.0 & Self::REQUEST) != 0
    }

    pub fn is_proxiable(&self) -> bool {
        (self.0 & Self::PROXIABLE) != 0
    }

    pub fn is_error(&self) -> bool {
        (self.0 & Self::ERROR) != 0
    }

    pub fn is_retransmitted(&self) -> bool {
        (self.0 & Self::RETRANSMITTED) != 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeaderError {
    Incomplete { needed: usize, available: usize },

    InvalidVersion(u8),

    InvalidLength(u32),

    UnalignedLength(u32),

    LengthExceedsMaximum { length: u32, maximum: u32 },

    InvalidFlags(u8),
}


impl fmt::Display for HeaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HeaderError::Incomplete { needed, available } => {
                write!(
                    f,
                    "Incomplete Diameter header: need {needed} bytes, received {available}"
                )
            }

            HeaderError::InvalidVersion(version) => {
                write!(f, "Unsupported Diameter version: {version}")
            }

            HeaderError::InvalidLength(length) => {
                write!(f, "Invalid Diameter message length: {length}")
            }

            HeaderError::UnalignedLength(length) => {
                write!(f, "Unaligned Diameter message length: {length}")
            }

            HeaderError::LengthExceedsMaximum { length, maximum } => {
                write!(
                    f,
                    "Diameter message length {length} exceeds maximum {maximum}"
                )
            }

            HeaderError::InvalidFlags(flags) => {
                write!(f, "Invalid Diameter command flags: {flags:#04X}")
            }
        }
    }
}

impl std::error::Error for HeaderError {}

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub struct DiameterHeader {
    pub version: u8,
    pub message_length: u32,
    pub flags: CommandFlags,
    pub command_code: u32,
    pub application_id: u32,
    pub hop_by_hop_id: u32,
    pub end_to_end: u32,
}

impl DiameterHeader {}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cer_handle() {
        let header = DiameterHeader {
            version: 1,
            message_length: 20,
            flags: CommandFlags::new(0x80),
            command_code: 257,
            application_id: 0,
            hop_by_hop_id: 1,
            end_to_end: 2,
        };

        assert_eq!(header.version, 1);
        assert_eq!(header.message_length, 20);
    }

    #[test]
    fn test_request_flag() {
        let flags = CommandFlags::new(0x80);

        assert!(flags.is_request());
    }

    #[test]
    fn test_invalid_length_error() {
        let error = HeaderError::InvalidLength(16);

        match error {
            HeaderError::InvalidLength(length) => {
                assert_eq!(length, 16);
            }
            _ => panic!("Expected InvalidLength"),
        }
    }

    #[test]
    fn test_command_flags() {
        let flags = CommandFlags::new(0xC0);

        assert!(flags.is_request());
        assert!(flags.is_proxiable());
        assert!(!flags.is_error());
        assert!(!flags.is_retransmitted());
    }

    #[test]
    fn test_header_error_display() {
        let error = HeaderError::InvalidLength(16);

        assert_eq!(error.to_string(), "Invalid Diameter message length: 16");
    }
}
