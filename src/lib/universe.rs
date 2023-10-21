use std::fmt::Display;

use crate::{mask, shift, SteamId};

/// Each universe is a self-contained Steam instance.
///
/// Meaning for every `account_number` inside the `Public` universe,
/// there exists an equivalent user inside the `Beta` or `Internal` universes.
///
/// You will almost always be interacting with the `Public` universe, however
/// for completeness all known universes are enumerated here.
#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord, Hash)]
pub enum Universe {
    /// Unspecified universe.
    ///
    /// This is ocassionally interchangable with Public instances, such as
    /// when parsing a SteamId2, in accordance with Valve's implementation.
    Unspecified,
    /// Public user accounts, this is what virtually all accounts are.
    Public,
    /// Internal Valve-only type.
    Beta,
    Internal,
    Dev,
    RC,
}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

// We can safely use a u8 here since Universe is only 8-bits in packed repr.
impl From<Universe> for u8 {
    #[rustfmt::skip]
    fn from(u: Universe) -> Self {
        use Universe::*;
        match u {
            Unspecified => 0,
            Public      => 1,
            Beta        => 2,
            Internal    => 3,
            Dev         => 4,
            RC          => 5,
        }
    }
}

impl From<u8> for Universe {
    #[rustfmt::skip]
    fn from(v: u8) -> Self {
        use Universe::*;
        match v {
            1   => Public,
            2   => Beta,
            3   => Internal,
            4   => Dev,
            5   => RC,
            _   => Unspecified,
        }
    }
}

impl From<SteamId> for Universe {
    #[rustfmt::skip]
    fn from(steamid: SteamId) -> Self {
        let universe_val = (steamid.id & mask::UNIVERSE) >> shift::UNIVERSE;
        // We can safely cast as u8 since Universe is 8-bits in packed repr.
        Universe::from(universe_val as u8)
    }
}

/////////////////////////////////////////////////////////////////////////////
// Unit Testing
/////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn value_conversion() {
        assert_eq!(Universe::from(0), Universe::Unspecified);
        assert_eq!(Universe::from(1), Universe::Public);
        assert_eq!(Universe::from(4), Universe::Dev);
        assert_eq!(Universe::from(100), Universe::Unspecified);
    }

    #[test]
    fn universe_fmt_debug() {
        for v in 1..=6 {
            let uv = Universe::from(v);
            let fmt = uv.to_string();
            let dbg = format!("{:?}", uv);
            assert!(fmt == dbg);
        }
    }
}
