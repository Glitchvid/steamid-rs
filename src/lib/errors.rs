//! Module to disambiguate our error-related types.
use std::fmt::{self, Debug, Display};

/// Parsing components of a SteamId
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Field {
    /// Authentication Server bit, only ever parsed directly from SteamId2.
    ///
    /// Values exceeding 1 fail during parsing.
    /// * STEAM_X:**Y**:Z
    AuthServer,
    /// 31-bit number, so values exceeding `2147483647` fail.
    ///
    /// Value is parsed directly from the Z value in a SteamId2,
    /// for SteamId3 the value is packed with the `AuthServer` bit.
    /// * STEAM_X:Y:**Z**
    /// * \[X:Y:**Z**]
    AccountNumber,
    Instance,
    /// Only directly parsed in SteamId3 formats.
    /// * \[**X**:Y:Z]
    AccountType,
    /// Almost always `1`.
    /// * STEAM_**X**:Y:Z
    /// * \[X:**Y**:Z]
    Universe,
    /// Failed to parse the value into a [u64].
    SteamId64,
}

impl Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Field::AuthServer => write!(f, "authentication server"),
            Field::AccountNumber => write!(f, "account number"),
            Field::Instance => write!(f, "instance"),
            Field::AccountType => write!(f, "account type"),
            Field::Universe => write!(f, "universe"),
            Field::SteamId64 => write!(f, "steamid64"),
        }
    }
}

/// Reasons why parsing a SteamId might fail.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum ParseError {
    /// Failed to deduce any SteamId format during parsing.
    UknownFormat,

    /// Failed to interpret a value during SteamId parsing.
    ///
    /// This is caused by SteamId being formatted incorrectly, such as having
    /// letters where numbers should be.
    Invalid(Field),

    /// Input did not contain all the fields required to parse.
    TooShort,

    /// Failed by having no data at all to parse.
    Empty,

    Other(&'static str),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UknownFormat => write!(f, "unable to identify SteamId format"),
            ParseError::Invalid(v) => write!(f, "invalid value in {v}"),
            ParseError::TooShort => write!(f, "unexpected end of string"),
            ParseError::Empty => write!(f, "input empty"),
            ParseError::Other(v) => write!(f, "{v}"),
        }
    }
}

impl std::error::Error for ParseError {}
